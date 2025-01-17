use crate::buffer::CryptoBuffer;
use crate::parse_buffer::ParseBuffer;
use crate::TlsError;
use heapless::Vec;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Certificate<'a> {
    entries: Vec<CertificateEntry<'a>, 16>,
}

impl<'a> Certificate<'a> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn parse(buf: &mut ParseBuffer<'a>) -> Result<Self, TlsError> {
        let request_context_len = buf.read_u8().map_err(|_| TlsError::InvalidCertificate)?;
        let _request_context = buf
            .slice(request_context_len as usize)
            .map_err(|_| TlsError::InvalidCertificate)?;
        let entries_len = buf.read_u24().map_err(|_| TlsError::InvalidCertificate)?;
        let mut entries = buf
            .slice(entries_len as usize)
            .map_err(|_| TlsError::InvalidCertificate)?;

        let entries = CertificateEntry::parse_vector(&mut entries)?;

        Ok(Self { entries })
    }

    pub(crate) fn encode(&self, buf: &mut CryptoBuffer<'_>) -> Result<(), TlsError> {
        // TODO: Implement
        buf.push(0).map_err(|_| TlsError::EncodeError)?;
        buf.extend_from_slice(&[0x00, 0x00, 0x00])
            .map_err(|_| TlsError::EncodeError)?;
        Ok(())
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CertificateEntry<'a> {
    X509(&'a [u8]),
    RawPublicKey(&'a [u8]),
}

impl<'a> CertificateEntry<'a> {
    pub fn parse_vector(
        buf: &mut ParseBuffer<'a>,
    ) -> Result<Vec<CertificateEntry<'a>, 16>, TlsError> {
        let mut entries = Vec::new();
        loop {
            let entry_len = buf.read_u24().map_err(|_| TlsError::InvalidCertificate)?;
            // info!("cert len: {}", entry_len);
            let cert = buf
                .slice(entry_len as usize)
                .map_err(|_| TlsError::InvalidCertificate)?;

            //let cert: Result<Vec<u8, _>, ()> = cert.into();
            // let cert: Result<Vec<u8, _>, ()> = Ok(Vec::new());

            entries
                .push(CertificateEntry::X509(cert.as_slice()))
                .map_err(|_| TlsError::DecodeError)?;

            let _extensions_len = buf
                .read_u16()
                .map_err(|_| TlsError::InvalidExtensionsLength)?;

            if buf.is_empty() {
                break;
            }
        }
        Ok(entries)
    }
}

impl<'a> Default for Certificate<'a> {
    fn default() -> Self {
        Certificate::new()
    }
}
