mod serialize;
mod deserialize;
mod test;

pub mod ser {
  pub use super::serialize::Error;
}

pub mod de {
  pub use super::deserialize::Error;
}

pub use serialize::BinSerializer;
pub use deserialize::BinDeserializer;

pub fn serialize< T: serde::Serialize >( val: &T ) -> Result< Vec< u8 >, ser::Error > {
  let mut s = BinSerializer::default( );
  val.serialize( &mut s )?;
  Ok( s.into( ) )
}

pub fn deserialize< T: for< 'de > serde::Deserialize< 'de > >( xs: &[u8] ) -> Result< T, de::Error > {
  let mut d = BinDeserializer::from( xs );
  T::deserialize( &mut d )
}
