use bytes::Bytes;

#[derive(Debug)]
pub enum Authentication {
    /// Authentication was successful.
    Ok,

    /// Kerberos V5 authentication is required.
    KerberosV5,

    /// A clear-text password is required.
    CleartextPassword,

    /// An MD5-encrypted password is required.
    Md5Password { salt: [u8; 4] },

    /// An SCM credentials message is required.
    ScmCredential,

    /// GSSAPI authentication is required.
    Gss,

    /// SSPI authentication is required.
    Sspi,

    /// This message contains GSSAPI or SSPI data.
    GssContinue { data: Bytes },

    /// SASL authentication is required.
    // FIXME: authentication mechanisms
    Sasl,

    /// This message contains a SASL challenge.
    SaslContinue { data: Bytes },

    /// SASL authentication has completed.
    SaslFinal { data: Bytes },
}