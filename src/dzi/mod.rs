use std::sync::Arc;

use custom_error::custom_error;
use dzi_file::DziFile;

use crate::dezoomer::*;
use crate::network::remove_bom;

mod dzi_file;

/// A dezoomer for Deep Zoom Images
/// See https://docs.microsoft.com/en-us/previous-versions/windows/silverlight/dotnet-windows-silverlight/cc645043%28v%3dvs.95%29
#[derive(Default)]
pub struct DziDezoomer;

impl Dezoomer for DziDezoomer {
    fn name(&self) -> &'static str {
        "deepzoom"
    }

    fn zoom_levels(&mut self, data: &DezoomerInput) -> Result<ZoomLevels, DezoomerError> {
        let DezoomerInputWithContents { uri, contents } = data.with_contents()?;
        let levels = load_from_properties(uri, contents)?;
        Ok(levels)
    }
}

custom_error! {pub DziError
    XmlError{source: serde_xml_rs::Error} = "Unable to parse the dzi file: {source}",
    NoSize = "Expected a size in the DZI file",
    InvalidTileSize = "Invalid tile size",
}

impl From<DziError> for DezoomerError {
    fn from(err: DziError) -> Self {
        DezoomerError::Other { source: err.into() }
    }
}

fn load_from_properties(url: &str, contents: &[u8]) -> Result<ZoomLevels, DziError> {

    // Workaround for https://github.com/netvl/xml-rs/issues/155
    // which the original author seems unwilling to fix
    let image_properties: DziFile = serde_xml_rs::from_reader(remove_bom(contents))?;

    if image_properties.tile_size == 0 {
        return Err(DziError::InvalidTileSize);
    }

    let dot_pos = url.rfind('.').unwrap_or(url.len() - 1);
    let mut url_str =  if image_properties.base_url.eq("no url") {  
	format!("{}_files", &url[0..dot_pos])
    } else {
	image_properties.base_url.clone()
    };
    if url_str.ends_with('/') {
	url_str.pop();
    }
    let base_url = &Arc::new(url_str);

    let size = image_properties.get_size()?;
    let max_level = image_properties.max_level();
    let levels = std::iter::successors(Some(size), |&size| {
        if size.x > 1 || size.y > 1 {
            Some(size.ceil_div(Vec2d::square(2)))
        } else {
            None
        }
    })
    .enumerate()
    .map(|(level_num, size)| DziLevel {
        base_url: Arc::clone(base_url),
        size,
        tile_size: image_properties.get_tile_size(),
        format: image_properties.format.clone(),
        overlap: image_properties.overlap,
        level: max_level - level_num as u32,
    })
    .into_zoom_levels();
    Ok(levels)
}

struct DziLevel {
    base_url: Arc<String>,
    size: Vec2d,
    tile_size: Vec2d,
    format: String,
    overlap: u32,
    level: u32,
}

impl TilesRect for DziLevel {
    fn size(&self) -> Vec2d {
        self.size
    }

    fn tile_size(&self) -> Vec2d {
        self.tile_size
    }

    fn tile_url(&self, pos: Vec2d) -> String {
        format!(
            "{base}/{level}/{x}_{y}.{format}",
            base = self.base_url,
            level = self.level,
            x = pos.x,
            y = pos.y,
            format = self.format
        )
    }

    fn tile_ref(&self, pos: Vec2d) -> TileReference {
        let delta = Vec2d {
            x: if pos.x == 0 { 0 } else { self.overlap },
            y: if pos.y == 0 { 0 } else { self.overlap },
        };
        TileReference {
            url: self.tile_url(pos),
            position: self.tile_size() * pos - delta,
        }
    }
}

impl std::fmt::Debug for DziLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Deep Zoom Image")
    }
}

#[test]
fn test_panorama() {
    let url = "http://x.fr/y/test.dzi";
    let contents = br#"
        <Image
          TileSize="256"
          Overlap="2"
          Format="jpg"
          >
          <Size Width="600" Height="300"/>
          <DisplayRects></DisplayRects>
        </Image>"#;
    let mut props = load_from_properties(url, contents).unwrap();
    assert_eq!(props.len(), 11);
    let level = &mut props[1];
    let tiles: Vec<String> = level.next_tiles(None).into_iter().map(|t| t.url).collect();
    assert_eq!(
        tiles,
        vec![
            "http://x.fr/y/test_files/9/0_0.jpg",
            "http://x.fr/y/test_files/9/1_0.jpg"
        ]
    );
}


#[test]
fn test_dzi_with_bom() {
    // See https://github.com/lovasoa/dezoomify-rs/issues/45
    // Trying to parse a file with a byte order mark
    let contents = "\u{feff}<?xml version=\"1.0\" encoding=\"utf-8\"?>
        <Image TileSize=\"256\" Overlap=\"0\" Format=\"jpg\" xmlns=\"http://schemas.microsoft.com/deepzoom/2008\">
        <Size Width=\"6261\" Height=\"6047\" />
        </Image>";
    load_from_properties("http://test.com/test.xml", contents.as_ref()).unwrap();
}
