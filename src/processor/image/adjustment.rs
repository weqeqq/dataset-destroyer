use crate::config::adjustment::*;
use super::{Image, ImageModifier};

use anyhow::Result;

impl ImageModifier for Brighten {
	fn apply(&self, image: &mut Image) -> Result<()> {
		let value = self.value()?;

		*image.source_mut() = image.source().brighten(value as i32);

		Ok(())
	}
	fn id(&self) -> &str {
		self.id.as_ref()
	}
}

impl ImageModifier for Contrast {
	fn apply(&self, image: &mut Image) -> Result<()> {
		let contrast = self.contrast()?;

		*image.source_mut() = image.source().adjust_contrast(contrast);

		Ok(())
	}
	fn id(&self) -> &str {
		self.id.as_ref()
	}
}
