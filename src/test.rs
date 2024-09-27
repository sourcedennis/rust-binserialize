
#[cfg(test)]
mod tests {
  use serde::Deserialize;
// external library imports
  use ::serde::Serialize;
  // local imports
  use crate::{BinSerializer, BinDeserializer};

  #[test]
  pub fn test_unit( ) {
    let v = ();
    let v_data: [u8; 0] = [];

    // # Serialize

    assert_eq!( bin_serialize( &v ), v_data );

    // # Deserialize

    assert_eq!( bin_deserialize::< () >( &v_data ), v );
  }

  #[test]
  pub fn test_unit_struct( ) {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct UnitStruct;

    let v = UnitStruct;
    let v_data: [u8; 0] = [];

    // # Serialize

    assert_eq!( bin_serialize( &v ), v_data );

    // # Deserialize

    assert_eq!( bin_deserialize::< UnitStruct >( &v_data ), v );
  }

  #[test]
  pub fn test_u32( ) {
    let v = 0x1234_5678u32;

    // # Serialize

    assert_eq!( bin_serialize( &v ), v.to_ne_bytes( ) );

    // # Deserialize

    assert_eq!( bin_deserialize::< u32 >( &v.to_ne_bytes( ) ), v );
  }

  #[test]
  pub fn test_u32_newtype( ) {
    #[derive(Serialize, Deserialize)]
    struct Meters( u32 );

    let v = 0x1234_5678u32;

    // # Serialize

    assert_eq!( bin_serialize( &Meters( v ) ), v.to_ne_bytes( ) );

    // # Deserialize

    assert_eq!( bin_deserialize::< Meters >( &v.to_ne_bytes( ) ).0, Meters( v ).0 );
  }

  #[test]
  pub fn test_u32_tuple( ) {
    let v1 = 0x1234_5678u32;
    let v2 = 0xBABE_EBABu32;

    // # Serialize

    let ser_data = concat::< _, 4, 4, 8 >( v1.to_ne_bytes( ), v2.to_ne_bytes( ) );
    assert_eq!( bin_serialize( &( v1, v2 ) ), ser_data );

    // # Deserialize

    assert_eq!( bin_deserialize::< ( u32, u32 ) >( &ser_data ), ( v1, v2 ) );
  }

  #[test]
  pub fn test_u32_struct( ) {
    #[derive(Serialize, Deserialize)]
    struct Meters { v: u32 }

    let v = 0x1234_5678u32;

    // # Serialize

    assert_eq!( bin_serialize( &Meters { v } ), v.to_ne_bytes( ) );

    // # Deserialize

    assert_eq!( bin_deserialize::< Meters >( &v.to_ne_bytes( ) ).v, v );
  }

  #[test]
  pub fn test_u32_tuple_struct( ) {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Meters2( u32, u32 );

    let v1 = 0x1234_5678u32;
    let v2 = 0xBABE_EBABu32;

    // # Serialize

    let data_raw = Meters2( v1, v2 );
    let data_ser = concat::< _, 4, 4, 8 >( v1.to_ne_bytes( ), v2.to_ne_bytes( ) );
    assert_eq!( bin_serialize( &data_raw ), data_ser );

    // # Deserialize

    assert_eq!( bin_deserialize::< Meters2 >( &data_ser ), data_raw );
  }

  #[test]
  pub fn test_enum( ) {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    enum Foo {
      Constr1( u32 ),
      Constr2( u64 ),
      Constr3,
      Constr12( u32, u64 )
    }

    let v1 = 0x1234_5678u32;
    let v2 = 0xBABE_EBAB_3355_77BBu64;

    let constr1_raw  = Foo::Constr1( v1 );
    let constr2_raw  = Foo::Constr2( v2 );
    let constr3_raw  = Foo::Constr3;
    let constr12_raw = Foo::Constr12( v1, v2 );

    let constr1_ser  = concat::< _, 1, 4, 5 >( [0], v1.to_ne_bytes( ) );
    let constr2_ser  = concat::< _, 1, 8, 9 >( [1], v2.to_ne_bytes( ) );
    let constr3_ser  = [2];
    let constr12_ser = concat::< _, 1, 12, 13 >( [3], concat::< _, 4, 8, 12 >( v1.to_ne_bytes( ), v2.to_ne_bytes( ) ) );

    // # Serialize

    assert_eq!( bin_serialize( &constr1_raw ), constr1_ser );
    assert_eq!( bin_serialize( &constr2_raw ), constr2_ser );
    assert_eq!( bin_serialize( &constr3_raw ), constr3_ser );
    assert_eq!( bin_serialize( &constr12_raw ), constr12_ser );

    // # Deserialize

    assert_eq!( bin_deserialize::< Foo >( &constr1_ser ), constr1_raw );
    assert_eq!( bin_deserialize::< Foo >( &constr2_ser ), constr2_raw );
    assert_eq!( bin_deserialize::< Foo >( &constr3_ser ), constr3_raw );
    assert_eq!( bin_deserialize::< Foo >( &constr12_ser ), constr12_raw );
  }

  #[test]
  pub fn test_vec_u32( ) {
    let xs = vec![ 1u32, 2, 3 ];

    let mut xs_ser = vec![ 0u8; 4 + 3 * 4 ];
    // len
    xs_ser[ 0..4 ].copy_from_slice( &3u32.to_ne_bytes( ) );
    // data
    xs_ser[ 4..8 ].copy_from_slice( &1u32.to_ne_bytes( ) );
    xs_ser[ 8..12 ].copy_from_slice( &2u32.to_ne_bytes( ) );
    xs_ser[ 12..16 ].copy_from_slice( &3u32.to_ne_bytes( ) );

    // # Serialize

    assert_eq!( bin_serialize( &xs ), xs_ser );

    // # Deserialize

    assert_eq!( bin_deserialize::< Vec< u32 > >( &xs_ser ), xs );
  }

  #[test]
  pub fn test_slice_u32( ) {
    let xs = vec![ 1u32, 2, 3 ];
    let xs = &xs;

    let mut xs_ser = vec![ 0u8; 4 + 3 * 4 ];
    // len
    xs_ser[ 0..4 ].copy_from_slice( &3u32.to_ne_bytes( ) );
    // data
    xs_ser[ 4..8 ].copy_from_slice( &1u32.to_ne_bytes( ) );
    xs_ser[ 8..12 ].copy_from_slice( &2u32.to_ne_bytes( ) );
    xs_ser[ 12..16 ].copy_from_slice( &3u32.to_ne_bytes( ) );

    // # Serialize

    assert_eq!( bin_serialize( &xs ), xs_ser );

    // # Deserialize

    assert_eq!( &bin_deserialize::< Vec< u32 > >( &xs_ser ), xs );
  }

  // # Helpers

  fn bin_serialize< V: Serialize >( v: &V ) -> Vec< u8 > {
    let mut s = BinSerializer::default( );
    v.serialize( &mut s ).unwrap( );
    s.into( )
  }

  fn bin_deserialize< V: for<'de> Deserialize< 'de > >( xs: &[u8] ) -> V {
    let mut d = BinDeserializer::from( xs );
    V::deserialize( &mut d ).unwrap( )
  }

  fn concat< T, const N1: usize, const N2: usize, const N12: usize >( xs: [T; N1], ys: [T; N2] ) -> [T; N12] {
    assert_eq!( N1 + N2, N12 );

    let mut iter = xs.into_iter( ).chain( ys );
    std::array::from_fn(|_| iter.next().unwrap())
  }
}
