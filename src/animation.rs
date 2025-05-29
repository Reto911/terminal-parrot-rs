use std::{collections::HashMap, fs, path::Path};

use anyhow::anyhow;

#[derive(Debug)]
pub struct Animation {
    pub metadata: HashMap<String, String>,
    pub frames: Vec<String>,
}

impl Animation {
    pub fn load_from_file(file: impl AsRef<Path>) -> anyhow::Result<Self> {
        let content = fs::read_to_string(file)?;
        Self::load_from_bytes(&content)
    }

    pub fn load_from_bytes(b: &str) -> anyhow::Result<Self> {
        let mut metadata = HashMap::new();
        let mut frames = vec![];
        let mut components = b.trim_start().split("!--FRAME--!\n");
        let meta = components.next().ok_or(anyhow!("No metadata found."))?;
        if meta.is_empty() {
            return Err(anyhow!("No metadata found."));
        }
        for line in meta.lines() {
            let (k, v) = line
                .split_once(":")
                .ok_or(anyhow!("Invalid metadata: `{line}`"))?;
            let k = k.trim();
            let v = v.trim();
            if k.is_empty() && v.is_empty() {
                return Err(anyhow!("Invalid metadata: `{line}`"));
            }
            metadata.insert(k.to_owned(), v.to_owned());
        }

        for (i, frame) in components.enumerate() {
            if frame.is_empty() {
                return Err(anyhow!("Invalid animation: frame {i} is empty."));
            }
            frames.push(frame.to_owned());
        }

        Ok(Self { metadata, frames })
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, io::Write};

    use tempfile::NamedTempFile;

    use super::Animation;

    const FRAME_VALID: &str = r#"
Description: Test frames.
Valid: True
!--FRAME--!
aaaaaaa
!--FRAME--!
bbbbbbb
"#;
    #[test]
    fn test_valid_animation() {
        let animation = Animation::load_from_bytes(FRAME_VALID).unwrap();
        assert_eq!(
            animation.metadata,
            HashMap::from([
                ("Description".to_owned(), "Test frames.".to_owned()),
                ("Valid".to_owned(), "True".to_owned())
            ])
        );
        assert_eq!(animation.frames, ["aaaaaaa\n", "bbbbbbb\n"])
    }

    #[test]
    fn test_valid_animation_from_file() {
        let mut tempfile = NamedTempFile::new().unwrap();
        tempfile.write_all(FRAME_VALID.as_bytes()).unwrap();
        tempfile.flush().unwrap();
        let animation = Animation::load_from_file(tempfile.path()).unwrap();
        assert_eq!(
            animation.metadata,
            HashMap::from([
                ("Description".to_owned(), "Test frames.".to_owned()),
                ("Valid".to_owned(), "True".to_owned())
            ])
        );
        assert_eq!(animation.frames, ["aaaaaaa\n", "bbbbbbb\n"])
    }

    #[test]
    #[should_panic(expected = "No metadata found.")]
    fn test_no_metadata() {
        let animation_invalid = "";
        Animation::load_from_bytes(animation_invalid).unwrap();
    }

    #[test]
    #[should_panic(expected = "No metadata found.")]
    fn test_no_metadata_2() {
        let animation_invalid = r#"
!--FRAME--!
aaaaaaa
!--FRAME--!
bbbbbbb
    "#;
        Animation::load_from_bytes(animation_invalid).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid metadata: `foobar`")]
    fn test_invalid_metadata() {
        let animation_invalid = r#"
foobar
!--FRAME--!
aaaaaaa
!--FRAME--!
bbbbbbb
"#;
        Animation::load_from_bytes(animation_invalid).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid animation")]
    fn invalid_frame() {
        let animation_invalid = r#"
Valid: False
!--FRAME--!
!--FRAME--!
bbbbbbb
"#;
        Animation::load_from_bytes(animation_invalid).unwrap();
    }
}
