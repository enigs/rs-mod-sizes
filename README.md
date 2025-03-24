# Image Sizing Library

A Rust library for handling image dimensions with different orientations and scales. This crate provides a robust system for creating, manipulating, and storing image size information.

## Features

- Three orientation types: Thumbnail (square), Landscape, and Portrait
- Seven scale options: from extra-extra-small to extra-extra-large
- Database integration with SQLx for Postgres
- Serialization/deserialization support with Serde
- Simple API for creating size objects

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sizes = { git = "https://github.com/enigs/rs-mod-sizes" }
```

## Usage

```rust
use sizes::{Size, Scale, Orientation, new_thumbnail, new_landscape, new_portrait};

// Create sizes using direct struct methods
let thumbnail = Size::new_thumbnail(64, Scale::Sm);
let banner = Size::new_landscape(1920, 1080, Scale::Lg);
let poster = Size::new_portrait(768, 1024, Scale::Md);

// Or use the convenience functions
let thumbnail = new_thumbnail(64, Scale::Sm);
let banner = new_landscape(1920, 1080, Scale::Lg);
let poster = new_portrait(768, 1024, Scale::Md);

// Check if a size is empty (default)
let empty_size = Size::default();
assert!(empty_size.is_empty());

// Convert orientation from string
let orientation = Orientation::from("landscape");
assert_eq!(orientation, Orientation::Landscape);

// Convert scale from string
let scale = Scale::from("lg");
assert_eq!(scale, Scale::Lg);
```

## Database Integration

The library integrates with SQLx for Postgres:

```rust
use sqlx::{Pool, Postgres};

async fn save_size(pool: &Pool<Postgres>, size: Size) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO images (size) VALUES ($1)")
        .bind(size)
        .execute(pool)
        .await?;
    
    Ok(())
}
```

## Scale Options

The library provides the following scale options:

| Scale | Description |
|-------|-------------|
| Xxsm  | Extra-extra-small |
| Xsm   | Extra-small |
| Sm    | Small |
| Md    | Medium |
| Lg    | Large |
| Xlg   | Extra-large |
| Xxlg  | Extra-extra-large |

## Orientation Types

Three orientation types are available:

- **Thumbnail**: Square aspect ratio (width equals height)
- **Landscape**: Width greater than height
- **Portrait**: Height greater than width

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.