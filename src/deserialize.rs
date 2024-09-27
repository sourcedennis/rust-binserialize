// stdlib imports
use std::str;
use std::fmt;
use std::mem::size_of;
// external library imports
use serde::Deserializer;
use serde::de::{self, EnumAccess, MapAccess, SeqAccess, VariantAccess};


#[derive(Default)]
pub struct BinDeserializer< 'de > {
  input: &'de [u8]
}

impl< 'de > From< &'de [u8] > for BinDeserializer< 'de > {
  #[inline]
  fn from( input: &'de [u8] ) -> Self {
    BinDeserializer { input }
  }
}

impl< 'de > BinDeserializer< 'de > {
  #[inline]
  fn peek_u8( &self ) -> Result< u8, Error > {
    if self.input.len( ) > 0 {
      Ok( self.input[ 0 ] )
    } else {
      Err( Error::InsufficientData )
    }
  }

  #[inline]
  fn peek_u8_n< const N: usize >( &self ) -> Result< &[u8], Error > {
    if self.input.len( ) >= N {
      Ok( &self.input[ 0..N ] )
    } else {
      Err( Error::InsufficientData )
    }
  }

  #[inline]
  fn peek_u8_nd( &self, n: usize ) -> Result< &[u8], Error > {
    if self.input.len( ) >= n {
      Ok( &self.input[ 0..n ] )
    } else {
      Err( Error::InsufficientData )
    }
  }

  #[inline]
  fn skip_unchecked( &mut self, n: usize ) {
    self.input = &self.input[ n.. ];
  }
}

impl< 'de, 'a > de::Deserializer< 'de > for &'a mut BinDeserializer< 'de > {
  type Error = Error;

  fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {
    // Our format is not self-describing, so there's no way to deserialize any
    // type.
    Err( Error::UnknownType )
  }

  #[inline]
  fn deserialize_bool<V>( self, visitor: V ) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {
    
    match self.peek_u8( )? {
      0 => {
        self.skip_unchecked( 1 );
        visitor.visit_bool( false )
      },
      1 => {
        self.skip_unchecked( 1 );
        visitor.visit_bool( true )
      },
      _ => Err( Error::MalformedBool )
    }
  }

  #[inline]
  fn deserialize_i8<V>( self, visitor: V ) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {
    
    let res = self.peek_u8( )?;
    self.skip_unchecked( 1 );
    visitor.visit_i8( unsafe { std::mem::transmute( res ) } )
  }

  #[inline]
  fn deserialize_i16<V>( self, visitor: V ) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {
    
    const N: usize = size_of::< u16 >( );
    let res = self.peek_u8_n::< N >( )?;
    let res = u16::from_ne_bytes( unsafe { res.try_into( ).unwrap_unchecked( ) } );
    self.skip_unchecked( N );
    visitor.visit_i16( unsafe { std::mem::transmute( res ) } )
  }

  #[inline]
  fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {

    const N: usize = size_of::< u32 >( );
    let res = self.peek_u8_n::< N >( )?;
    let res = u32::from_ne_bytes( unsafe { res.try_into( ).unwrap_unchecked( ) } );
    self.skip_unchecked( N );
    visitor.visit_i32( unsafe { std::mem::transmute( res ) } )
  }

  #[inline]
  fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {

    const N: usize = size_of::< u64 >( );
    let res = self.peek_u8_n::< N >( )?;
    let res = u64::from_ne_bytes( unsafe { res.try_into( ).unwrap_unchecked( ) } );
    self.skip_unchecked( N );
    visitor.visit_i64( unsafe { std::mem::transmute( res ) } )
  }

  #[inline]
  fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {

    let res = self.peek_u8( )?;
    self.skip_unchecked( 1 );
    visitor.visit_u8( res )
  }

  #[inline]
  fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {

    const N: usize = size_of::< u16 >( );
    let res = self.peek_u8_n::< N >( )?;
    let res = u16::from_ne_bytes( unsafe { res.try_into( ).unwrap_unchecked( ) } );
    self.skip_unchecked( N );
    visitor.visit_u16( res )
  }

  #[inline]
  fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {

    const N: usize = size_of::< u32 >( );
    let res = self.peek_u8_n::< N >( )?;
    let res = u32::from_ne_bytes( unsafe { res.try_into( ).unwrap_unchecked( ) } );
    self.skip_unchecked( N );
    visitor.visit_u32( res )
  }

  #[inline]
  fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {

    const N: usize = size_of::< u64 >( );
    let res = self.peek_u8_n::< N >( )?;
    let res = u64::from_ne_bytes( unsafe { res.try_into( ).unwrap_unchecked( ) } );
    self.skip_unchecked( N );
    visitor.visit_u64( res )
  }

  #[inline]
  fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {
        
    const N: usize = size_of::< u32 >( );
    let res = self.peek_u8_n::< N >( )?;
    let res = u32::from_ne_bytes( unsafe { res.try_into( ).unwrap_unchecked( ) } );
    self.skip_unchecked( N );
    visitor.visit_f32( unsafe { std::mem::transmute( res ) } )
  }

  #[inline]
  fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {

    const N: usize = size_of::< u64 >( );
    let res = self.peek_u8_n::< N >( )?;
    let res = u64::from_ne_bytes( unsafe { res.try_into( ).unwrap_unchecked( ) } );
    self.skip_unchecked( N );
    visitor.visit_f64( unsafe { std::mem::transmute( res ) } )
  }

  #[inline]
  fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {

    let b0 = self.peek_u8( )?;
    let n = utf8_byte_len( b0 ).ok_or( Error::MalformedUtf8 )?;
    let bytes = self.peek_u8_nd( n )?;
    let res = str::from_utf8( bytes ).map_err( |_| Error::MalformedUtf8 )?;
    let res_char = res.chars( ).next( ).unwrap( );
    self.skip_unchecked( n );
    visitor.visit_char( res_char )
  }

  #[inline]
  fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {

    // len
    const N: usize = size_of::< u32 >( );
    let len = self.peek_u8_n::< N >( )?;
    let len = u32::from_ne_bytes( unsafe { len.try_into( ).unwrap_unchecked( ) } );
    self.skip_unchecked( N );

    // payload
    let data = self.peek_u8_nd( len as usize )?;
    let data_str = str::from_utf8( data ).map_err( |_| Error::MalformedUtf8 )?;
    let res = visitor.visit_str( data_str );
    self.skip_unchecked( len as usize );
    res
  }

  #[inline]
  fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {

    // len
    const N: usize = size_of::< u32 >( );
    let len = self.peek_u8_n::< N >( )?;
    let len = u32::from_ne_bytes( unsafe { len.try_into( ).unwrap_unchecked( ) } );
    self.skip_unchecked( N );

    // payload
    let data = self.peek_u8_nd( len as usize )?;
    let data_str = String::from_utf8( data.to_owned( ) ).map_err( |_| Error::MalformedUtf8 )?;
    let res = visitor.visit_string( data_str );
    self.skip_unchecked( len as usize );
    res
  }

  #[inline]
  fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {

    // len
    const N: usize = size_of::< u64 >( );
    let len = self.peek_u8_n::< N >( )?;
    let len = u64::from_ne_bytes( unsafe { len.try_into( ).unwrap_unchecked( ) } );
    self.skip_unchecked( N );

    // payload
    let data = self.peek_u8_nd( len as usize )?;
    let res = visitor.visit_bytes( data );
    self.skip_unchecked( len as usize );
    res
  }

  #[inline]
  fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {

    // len
    const N: usize = size_of::< u64 >( );
    let len = self.peek_u8_n::< N >( )?;
    let len = u64::from_ne_bytes( unsafe { len.try_into( ).unwrap_unchecked( ) } );
    self.skip_unchecked( N );

    // payload
    let data = self.peek_u8_nd( len as usize )?;
    let res = visitor.visit_byte_buf( data.to_owned( ) );
    self.skip_unchecked( len as usize );
    res
  }

  #[inline]
  fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {

    let marker = self.peek_u8( )?;

    match marker {
      0 => { // None
        self.skip_unchecked( 1 );
        visitor.visit_none( )
      },
      1 => { // Some
        self.skip_unchecked( 1 );
        visitor.visit_some( self )
      },
      _ => {
        Err( Error::MalformedOption )
      }
    }
  }

  #[inline]
  fn deserialize_unit< V >( self, visitor: V ) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {

    visitor.visit_unit( )
  }

  #[inline]
  fn deserialize_unit_struct< V >(
    self
  , _name: &'static str
  , visitor: V,
  ) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {
    
    self.deserialize_unit( visitor )
  }

  #[inline]
  fn deserialize_newtype_struct<V>(
    self
  , _name: &'static str
  , visitor: V
  ) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {
    
    visitor.visit_newtype_struct( self )
  }

  #[inline]
  fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {
    
    const N: usize = size_of::< u32 >( );
    let res = self.peek_u8_n::< N >( )?;
    let len = u32::from_ne_bytes( unsafe { res.try_into( ).unwrap_unchecked( ) } );
    self.skip_unchecked( N );
    
    visitor.visit_seq( Sequenced { de: self, len: len as usize } )
  }

  #[inline]
  fn deserialize_tuple<V>(
    self
  , len: usize
  , visitor: V
  ) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {
    
    visitor.visit_seq( Sequenced { de: self, len } )
  }

  #[inline]
  fn deserialize_tuple_struct<V>(
    self
  , _name: &'static str
  , len: usize
  , visitor: V
  ) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {

    visitor.visit_seq( Sequenced { de: self, len } )
  }

  #[inline]
  fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {
    
    // len
    const N: usize = size_of::< u32 >( );
    let len = self.peek_u8_n::< N >( )?;
    let len = u32::from_ne_bytes( unsafe { len.try_into( ).unwrap_unchecked( ) } );
    self.skip_unchecked( N );

    visitor.visit_map( Sequenced { de: self, len: len as usize } )
  }

  #[inline]
  fn deserialize_struct<V>(
    self
  , _name: &'static str
  , fields: &'static [&'static str]
  , visitor: V
  ) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {
    
    visitor.visit_seq( Sequenced { de: self, len: fields.len( ) } )
  }

  #[inline]
  fn deserialize_enum<V>(
    self
  , _name: &'static str
  , _variants: &'static [&'static str]
  , visitor: V
  ) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {

    visitor.visit_enum( Enum { de: self } )
  }

  #[inline]
  fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {

    self.deserialize_u8( visitor )
  }

  #[inline]
  fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {
    self.deserialize_any( visitor )
  }
}

struct Sequenced< 'a, 'de: 'a > {
  de: &'a mut BinDeserializer< 'de >,
  len: usize
}

impl< 'de, 'a > SeqAccess< 'de > for Sequenced< 'a, 'de > {
  type Error = Error;

  #[inline]
  fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
      T: de::DeserializeSeed<'de> {
    
    if self.len == 0 {
      Ok( None )
    } else {
      let val = seed.deserialize( &mut *self.de )?;
      self.len -= 1;
      Ok( Some( val ) )
    }
  }
}

impl< 'de, 'a > MapAccess< 'de > for Sequenced< 'a, 'de > {
  type Error = Error;

  #[inline]
  fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
      K: de::DeserializeSeed<'de> {
    
    if self.len == 0 {
      Ok( None )
    } else {
      let val = seed.deserialize( &mut *self.de )?;
      self.len -= 1;
      Ok( Some( val ) )
    }
  }

  #[inline]
  fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
      V: de::DeserializeSeed<'de> {

    seed.deserialize( &mut *self.de )
  }
}


struct Enum< 'a, 'de: 'a > {
  de: &'a mut BinDeserializer< 'de >
}

impl< 'a, 'de > EnumAccess< 'de > for Enum< 'a, 'de > {
  type Error = Error;

  type Variant = Self;

  #[inline]
  fn variant_seed<V>( self, seed: V ) -> Result<(V::Value, Self::Variant), Self::Error>
    where
      V: de::DeserializeSeed<'de> {
    let idx = seed.deserialize( &mut *self.de )?;
    Ok( ( idx, self ) )
  }
}

impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
  type Error = Error;

  #[inline]
  fn unit_variant(self) -> Result<(), Self::Error> {
    Ok( () )
  }

  #[inline]
  fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
      T: de::DeserializeSeed<'de> {

    seed.deserialize( self.de )
  }

  #[inline]
  fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {
    
    self.de.deserialize_tuple( len, visitor )
  }

  #[inline]
  fn struct_variant<V>(
    self
  , _fields: &'static [&'static str]
  , visitor: V
  ) -> Result<V::Value, Self::Error>
    where
      V: de::Visitor<'de> {
    
    self.de.deserialize_map( visitor )
  }
}

/// Helper. Returns the number of bytes needed for a char's UTF-8 enconding,
/// based on the first byte in the sequence.
#[inline]
fn utf8_byte_len( b0: u8 ) -> Option< usize > {
  if ( b0 & 0b1000_0000 ) == 0 {
    Some( 1 )
  } else if ( b0 & 0b1110_0000 ) == 0b1100_0000 {
    Some( 2 )
  } else if ( b0 & 0b1111_0000 ) == 0b1110_0000 {
    Some( 3 )
  } else if ( b0 & 0b1111_1000 ) == 0b1111_0000 {
    Some( 4 )
  } else {
    None
  }
}

/// Deserialization error
/// 
/// 
/// # Design Decision: Different serialization error
/// 
/// Errors where a length exceeds a pre-defined bound can only happen during
/// serialization. If serialization succeeded, the value is within that bound,
/// and the same error cannot occur during deserialization. Hence, we model them
/// differently.
#[derive(Debug)]
pub enum Error {
  Custom( String ),
  UnknownType,
  MalformedBool,
  MalformedUtf8,
  MalformedOption,
  InsufficientData,
  UnknownSeqLen,
  UnknownEnumVariant
}

impl fmt::Display for Error {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::Custom( msg ) => {
        write!( f, "{}", msg )
      },
      Error::UnknownType => {
        write!( f, "UnknownType" )
      },
      Error::MalformedBool => {
        write!( f, "MalformedBool" )
      },
      Error::MalformedUtf8 => {
        write!( f, "MalformedUtf8" )
      },
      Error::MalformedOption => {
        write!( f, "MalformedNone" )
      },
      Error::InsufficientData => {
        write!( f, "InsufficientData" )
      },
      Error::UnknownSeqLen => {
        write!( f, "UnknownSeqLen" )
      },
      Error::UnknownEnumVariant => {
        write!( f, "UnknownEnumVariant" )
      }
    }
  }
}

impl std::error::Error for Error { }

impl de::Error for Error {
  #[inline]
  fn custom<T>( msg: T ) -> Self where T: fmt::Display {
    Error::Custom( msg.to_string( ) )
  }
}
