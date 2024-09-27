// stdlib imports
use std::fmt;
// external library imports
use serde::{ser, Serialize};


#[derive(Default)]
pub struct BinSerializer {
  out: Vec< u8 >
}

impl From< BinSerializer > for Vec< u8 > {
  #[inline]
  fn from( v: BinSerializer ) -> Self {
    v.out
  }
}

impl< 'a > ser::Serializer for &'a mut BinSerializer {
  type Ok = ();

  type Error = Error;

  type SerializeSeq = Self;
  type SerializeTuple = Self;
  type SerializeTupleStruct = Self;
  type SerializeTupleVariant = Self;
  type SerializeMap = Self;
  type SerializeStruct = Self;
  type SerializeStructVariant = Self;

  #[inline]
  fn serialize_bool(self, v: bool) -> Result< Self::Ok, Self::Error > {
    self.out.push( if v { 1 } else { 0 } );
    Ok( () )
  }

  #[inline]
  fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
    self.serialize_u8( unsafe { std::mem::transmute( v ) } )
  }

  #[inline]
  fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
    self.serialize_u16( unsafe { std::mem::transmute( v ) } )
  }

  #[inline]
  fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
    self.serialize_u32( unsafe { std::mem::transmute( v ) } )
  }

  #[inline]
  fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
    self.serialize_u64( unsafe { std::mem::transmute( v ) } )
  }

  #[inline]
  fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
    self.out.push( v );
    Ok( () )
  }

  #[inline]
  fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
    self.out.extend_from_slice( &v.to_ne_bytes( ) );
    Ok( () )
  }

  #[inline]
  fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
    self.out.extend_from_slice( &v.to_ne_bytes( ) );
    Ok( () )
  }

  #[inline]
  fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
    self.out.extend_from_slice( &v.to_ne_bytes( ) );
    Ok( () )
  }

  #[inline]
  fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
    self.serialize_u32( unsafe { std::mem::transmute( v ) } )
  }

  #[inline]
  fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
    self.serialize_u64( unsafe { std::mem::transmute( v ) } )
  }

  #[inline]
  fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
    let mut buffer = [0u8; 4];
    let buffer_slice = v.encode_utf8( &mut buffer );
    let num_bytes = buffer_slice.len( );
    // drop: buffer_slice
    self.out.extend_from_slice( &buffer[0..num_bytes] );
    Ok( () )
  }

  #[inline]
  fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
    let len = v.len( ); // Length in *bytes*
    if len > ( u32::MAX as usize ) {
      return Err( Error::ExceedStringLen );
    }
    self.serialize_u32( len as u32 )?;
    self.out.extend_from_slice( v.as_bytes( ) );
    Ok( () )
  }

  #[inline]
  fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
    let len = v.len( ); // Length in *bytes*
    if len > ( u64::MAX as usize ) {
      return Err( Error::ExceedBytesLen );
    }
    self.serialize_u64( len as u64 )?;
    self.out.extend_from_slice( v );
    Ok( () )
  }

  #[inline]
  fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
    self.serialize_u8( 0 )
  }

  #[inline]
  fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
      T: ?Sized + Serialize {

    self.serialize_u8( 1 )?;
    value.serialize( self )
  }

  #[inline]
  fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
    Ok( () )
  }

  #[inline]
  fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
    self.serialize_unit( )
  }

  #[inline]
  fn serialize_unit_variant(
    self,
    _name: &'static str,
    variant_index: u32,
    _variant: &'static str,
  ) -> Result<Self::Ok, Self::Error> {
    if variant_index > ( u8::MAX as u32 ) {
      Err( Error::ExceedEnumVariant )
    } else {
      self.serialize_u8( variant_index as u8 )?;
      Ok( () )
    }
  }

  #[inline]
  fn serialize_newtype_struct<T>(
    self,
    _name: &'static str,
    value: &T,
  ) -> Result<Self::Ok, Self::Error>
    where
      T: ?Sized + Serialize {
    
    value.serialize( self )
  }

  #[inline]
  fn serialize_newtype_variant<T>(
      self,
      _name: &'static str,
      variant_index: u32,
      _variant: &'static str,
      value: &T,
  ) -> Result<Self::Ok, Self::Error>
    where
      T: ?Sized + Serialize {

    if variant_index > ( u8::MAX as u32 ) {
      Err( Error::ExceedEnumVariant )
    } else {
      self.serialize_u8( variant_index as u8 )?;
      value.serialize( self )
    }
  }

  #[inline]
  fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
    if let Some( len ) = len {
      if len <= ( u32::MAX as usize ) {
        self.serialize_u32( len as u32 )?;
        Ok( self )
      } else {
        Err( Error::ExceedSeqLen )
      }
    } else {
      Err( Error::UnknownSeqLen )
    }
  }

  #[inline]
  fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
    // Note that the tuple length is statically known
    Ok( self )
  }

  #[inline]
  fn serialize_tuple_struct(
    self
  , _name: &'static str
  , _len: usize
  ) -> Result<Self::SerializeTupleStruct, Self::Error> {
    // Note that the tuple length is statically known
    Ok( self )
  }

  #[inline]
  fn serialize_tuple_variant(
    self
  , _name: &'static str
  , variant_index: u32
  , _variant: &'static str
  , _len: usize,
  ) -> Result<Self::SerializeTupleVariant, Self::Error> {
    if variant_index > ( u8::MAX as u32 ) {
      Err( Error::ExceedEnumVariant )
    } else {
      self.serialize_u8( variant_index as u8 )?;
      // Note that the tuple length is statically known
      Ok( self )
    }
  }

  #[inline]
  fn serialize_map( self, len: Option<usize> ) -> Result<Self::SerializeMap, Self::Error> {
    if let Some( len ) = len {
      if len <= u32::MAX as usize {
        self.serialize_u32( len as u32 )?;
        Ok( self )
      } else {
        Err( Error::ExceedMapLen )
      }
    } else {
      Err( Error::UnknownMapLen )
    }
  }

  #[inline]
  fn serialize_struct(
    self,
    _name: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStruct, Self::Error> {
    Ok( self )
  }

  #[inline]
  fn serialize_struct_variant(
    self,
    _name: &'static str,
    variant_index: u32,
    _variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStructVariant, Self::Error> {
    if variant_index > ( u8::MAX as u32 ) {
      Err( Error::ExceedEnumVariant )
    } else {
      self.serialize_u8( variant_index as u8 )?;
      Ok( self )
    }
  }
}

impl<'a> ser::SerializeSeq for &'a mut BinSerializer {
  type Ok = ();
  type Error = Error;

  #[inline]
  fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
      T: ?Sized + Serialize {
    value.serialize( &mut **self )
  }

  #[inline]
  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok( () )
  }
}

impl<'a> ser::SerializeTuple for &'a mut BinSerializer {
  type Ok = ();
  type Error = Error;
  
  #[inline]
  fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
      T: ?Sized + Serialize {
    value.serialize( &mut **self )
  }
  
  #[inline]
  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok( () )
  }
}

impl<'a> ser::SerializeTupleStruct for &'a mut BinSerializer {
  type Ok = ();
  type Error = Error;
  
  #[inline]
  fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
      T: ?Sized + Serialize {
    value.serialize( &mut **self )
  }
  
  #[inline]
  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok( () )
  }
}

impl<'a> ser::SerializeTupleVariant for &'a mut BinSerializer {
  type Ok = ();
  type Error = Error;
  
  #[inline]
  fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
      T: ?Sized + Serialize {
    value.serialize( &mut **self )
  }
  
  #[inline]
  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok( () )
  }
}

impl<'a> ser::SerializeMap for &'a mut BinSerializer {
  type Ok = ();
  type Error = Error;

  #[inline]
  fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
      T: ?Sized + Serialize {
    key.serialize( &mut **self )
  }

  #[inline]
  fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
      T: ?Sized + Serialize {
    value.serialize( &mut **self )
  }

  #[inline]
  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok( () )
  }
}

impl<'a> ser::SerializeStruct for &'a mut BinSerializer {
  type Ok = ();
  type Error = Error;

  #[inline]
  fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
    where
      T: ?Sized + Serialize {
    value.serialize( &mut **self )
  }

  #[inline]
  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok( () )
  }
}

impl<'a> ser::SerializeStructVariant for &'a mut BinSerializer {
  type Ok = ();
  type Error = Error;

  #[inline]
  fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
    where
      T: ?Sized + Serialize {
    value.serialize( &mut **self )
  }

  #[inline]
  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok( () )
  }
}

/// Serialization error
/// 
/// 
/// # Design Decision: Different deserialization error
/// 
/// Errors where a length exceeds a pre-defined bound can only happen during
/// serialization. If serialization succeeded, the value is within that bound,
/// and the same error cannot occur during deserialization. Hence, we model them
/// differently.
#[derive(Debug)]
pub enum Error {
  /// The length of a string exceeds 2^32-1
  ExceedStringLen,
  /// The length of a map exceeds 2^32-1
  ExceedMapLen,
  /// The length of a byte array exceeds 2^64-1
  ExceedBytesLen,
  /// The enum variant index exceeded 2^8-1
  ExceedEnumVariant,
  /// The sequence length exceeds 2^32-1
  ExceedSeqLen,
  UnknownSeqLen,
  UnknownMapLen,
  Custom( String )
}

impl fmt::Display for Error {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::ExceedStringLen => {
        write!( f, "ExceedStringLen" )
      },
      Error::ExceedMapLen => {
        write!( f, "ExceedMapLen" )
      },
      Error::ExceedBytesLen => {
        write!( f, "ExceedBytesLen" )
      },
      Error::ExceedSeqLen => {
        write!( f, "ExceedSeqLen" )
      },
      Error::ExceedEnumVariant => {
        write!( f, "ExceedEnumVariant" )
      },
      Error::UnknownSeqLen => {
        write!( f, "UnknownSeqLen" )
      },
      Error::UnknownMapLen => {
        write!( f, "UnknownMapLen" )
      },
      Error::Custom( msg ) => {
        write!( f, "{}", msg )
      }
    }
  }
}

impl std::error::Error for Error { }

impl ser::Error for Error {
  #[inline]
  fn custom<T>( msg: T ) -> Self where T: fmt::Display {
    Error::Custom( msg.to_string( ) )
  }
}
