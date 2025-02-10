use anyhow::Result;
use std::{fs, io::Write, path::PathBuf};

mod tables;

pub const SCALES_FILE_EXTENSION: &str = "acss";
pub const COEFFICIENTS_FILE_EXTENSION: &str = "acsc";

fn write_bytes(path: &PathBuf, bytes: &[u8]) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)?;

    file.write_all(bytes)?;

    Ok(())
}

#[allow(clippy::needless_range_loop)]
fn flatten_coefficients(data: &[[[[[f32; 3]; 64]; 64]; 64]; 3]) -> Vec<f32> {
    let mut result = Vec::with_capacity(3 * 64 * 64 * 64 * 3);
    for i in 0..3 {
        for j in 0..64 {
            for k in 0..64 {
                for l in 0..64 {
                    for m in 0..3 {
                        result.push(data[i][j][k][l][m]);
                    }
                }
            }
        }
    }
    result
}

#[cfg(feature = "aces")]
pub fn write_aces_tables(path: PathBuf) -> Result<()> {
    let mut scales_path = path.clone();
    scales_path.set_extension(SCALES_FILE_EXTENSION);
    write_bytes(
        &scales_path,
        bytemuck::cast_slice(&tables::aces_to_spectrum::ACES_TO_SPECTRUM_TABLE_SCALE),
    )?;

    let mut coeffs_path = path.clone();
    coeffs_path.set_extension(COEFFICIENTS_FILE_EXTENSION);
    write_bytes(
        &coeffs_path,
        bytemuck::cast_slice(&flatten_coefficients(
            &tables::aces_to_spectrum::ACES_TO_SPECTRUM_TABLE_DATA,
        )),
    )?;

    Ok(())
}

#[cfg(feature = "dci_p3")]
pub fn write_dci_p3_tables(path: PathBuf) -> Result<()> {
    let mut scales_path = path.clone();
    scales_path.set_extension(SCALES_FILE_EXTENSION);
    write_bytes(
        &scales_path,
        bytemuck::cast_slice(&tables::dci_p3_to_spectrum::DCI_P3_TO_SPECTRUM_TABLE_SCALE),
    )?;

    let mut coeffs_path = path.clone();
    coeffs_path.set_extension(COEFFICIENTS_FILE_EXTENSION);
    write_bytes(
        &coeffs_path,
        bytemuck::cast_slice(&flatten_coefficients(
            &tables::dci_p3_to_spectrum::DCI_P3_TO_SPECTRUM_TABLE_DATA,
        )),
    )?;

    Ok(())
}

#[cfg(feature = "rec2020")]
pub fn write_rec2020_tables(path: PathBuf) -> Result<()> {
    let mut scales_path = path.clone();
    scales_path.set_extension(SCALES_FILE_EXTENSION);
    write_bytes(
        &scales_path,
        bytemuck::cast_slice(&tables::rec2020_to_spectrum::REC2020_TO_SPECTRUM_TABLE_SCALE),
    )?;

    let mut coeffs_path = path.clone();
    coeffs_path.set_extension(COEFFICIENTS_FILE_EXTENSION);
    write_bytes(
        &coeffs_path,
        bytemuck::cast_slice(&flatten_coefficients(
            &tables::rec2020_to_spectrum::REC2020_TO_SPECTRUM_TABLE_DATA,
        )),
    )?;

    Ok(())
}

#[cfg(feature = "srgb")]
pub fn write_srgb_tables(path: PathBuf) -> Result<()> {
    let mut scales_path = path.clone();
    scales_path.set_extension(SCALES_FILE_EXTENSION);
    write_bytes(
        &scales_path,
        bytemuck::cast_slice(&tables::srgb_to_spectrum::SRGB_TO_SPECTRUM_TABLE_SCALE),
    )?;

    let mut coeffs_path = path.clone();
    coeffs_path.set_extension(COEFFICIENTS_FILE_EXTENSION);
    write_bytes(
        &coeffs_path,
        bytemuck::cast_slice(&flatten_coefficients(
            &tables::srgb_to_spectrum::SRGB_TO_SPECTRUM_TABLE_DATA,
        )),
    )?;

    Ok(())
}
