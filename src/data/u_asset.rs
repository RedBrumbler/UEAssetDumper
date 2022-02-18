use super::{content_item::ContentItem, header_item::HeaderItem, int_item::IntItem, linked_item::LinkedItem, member_item::MemberItem, string_item::StringItem, umap_item::UmapItem};
use std::{path::PathBuf};
use std::io::{Cursor, Read};
use core::convert::TryInto;

use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Asset {
    pub head_vec: Vec<HeaderItem>,
    pub link_vec: Vec<LinkedItem>,
    pub cat_vec: Vec<MemberItem>,
    pub int_vec: Vec<IntItem>,
    pub string_vec: Vec<StringItem>,
    pub umap: UmapItem,
    pub content_vec: Vec<ContentItem>,

    #[serde(skip_serializing)]
    pub bytes: Vec<u8>,
    pub u_exp: i32,
    pub v_type: i32,
    #[serde(skip_serializing)]
    pub fun_error_count: i32,
    #[serde(skip_serializing)]
    pub cat_err_count: i32,
    #[serde(skip_serializing)]
    pub array_err_count: i32,
    #[serde(skip_serializing)]
    pub struct_err_count: i32,
    pub g: i32,
    pub gap_size: i32,
    pub header_size: i32,
}

pub trait CursorUtils : Read {
    fn read_bytes(&mut self, size: usize) -> Vec<u8> {
        let mut i = Vec::<u8>::with_capacity(size);
        self.read_exact(i.as_mut_slice()).expect("Couldn't read bytes");
        i
    }
    
    fn read_byte(&mut self) -> u8 {
        let mut i = [0];
        self.read_exact(&mut i).expect("Couldn't read bytes");
        i[0]
    }

    fn read_i32(&mut self) -> i32 {
        let mut i = [0; std::mem::size_of::<i32>()];
        self.read_exact(&mut i).expect("Couldn't read bytes");
        i32::from_le_bytes(i)
    }

    fn read_i16(&mut self) -> i16 {
        let mut i = [0; std::mem::size_of::<i16>()];
        self.read_exact(&mut i).expect("Couldn't read bytes");
        i16::from_le_bytes(i)
    }

    fn read_u32(&mut self) -> u32 {
        let mut i = [0; std::mem::size_of::<u32>()];
        self.read_exact(&mut i).expect("Couldn't read bytes");
        u32::from_le_bytes(i)
    }

    fn read_u16(&mut self) -> u16 {
        let mut i = [0; std::mem::size_of::<u16>()];
        self.read_exact(&mut i).expect("Couldn't read bytes");
        u16::from_le_bytes(i)
    }

    fn read_prefixed_string(&mut self) -> String {
        let count = self.read_i32();
        let mut tmp = if count >= 0 {
            vec![0u8; count as usize]
        } else {
            vec![0u8; (count * -2) as usize]
        };
        self.read_exact(tmp.as_mut_slice()).expect("Couldn't read bytes");
        let result = String::from_utf8(tmp).unwrap();
        result[0..result.len() - 1].to_string()
    }
}

impl<T: Read> CursorUtils for T {}

impl Asset {
    pub fn get_bytes_i32(&self, i: usize) -> i32 {
        i32::from_le_bytes(self.bytes[i..i + 4].try_into().unwrap())
    }

    pub fn parse_file_bytes(&mut self, bytes: Vec<u8>, file_path: PathBuf)
    {
        self.u_exp = 0;
        self.fun_error_count = 0;
        self.cat_err_count = 0;
        self.bytes = bytes;

        let magic = self.get_bytes_i32(0);
        if  magic != -1641380927 {
            self.bytes = Default::default();
            println!("Didn't find the right first byte: {}", magic);
            return;
        }
        if *self.bytes.get(8).unwrap() != 0_u8 {
            let n_20 = self.bytes.get(20).unwrap();
            self.g = (*n_20 as i32) * 20;
        }

        
        let num1 = self.get_bytes_i32(24 + self.g as usize);
        let num2 = self.get_bytes_i32(41 + self.g as usize);
        self.header_size = self.get_bytes_i32(45 + self.g as usize);
        let num3 = self.get_bytes_i32(57 + self.g as usize);
        let num4 = self.get_bytes_i32(65 + self.g as usize);
        let mut num5 = self.get_bytes_i32(73 + self.g as usize);
        let num6 = self.get_bytes_i32(77 + self.g as usize);
        let mut num7 = 0;

        if self.header_size == 185 || self.g > 0 {
            self.v_type = 1;
            num7 = self.get_bytes_i32(165 + self.g as usize);
        }
        if self.header_size == 193 {
            if num5 == 0 {
                num5 = self.get_bytes_i32(189 + self.g as usize);
                self.v_type = 3;
            } else {
                self.v_type = 2;
            }
        }
        if self.header_size == 197 {
            self.gap_size = self.get_bytes_i32(165 + self.g as usize) - num5;
            self.v_type = 2;
        }

        if file_path.with_extension("uexp").exists()
        {
            self.u_exp = self.bytes.len() as i32;
            let mut file = std::fs::File::open(file_path.with_extension("uexp")).expect("Failed to open uexp file");
            let mut uexp_buffer = Cursor::new(Vec::new());
            file.read_to_end(uexp_buffer.get_mut()).expect("Failed to read file");
            self.bytes.append(uexp_buffer.get_mut());
        }
        
        let mut byte_cursor = Cursor::new(&self.bytes);
        byte_cursor.set_position(self.header_size as u64);
        println!("header size: {}", self.header_size);
        for _ in 0..num2 {
            let name = byte_cursor.read_prefixed_string();
            let code = byte_cursor.read_u32();
            self.head_vec.push(HeaderItem::new(name, code));
        }
        for _ in 0..num4 {
            self.link_vec.push(LinkedItem::new(byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32()));
        }

        for _ in 0..num3 {
            self.cat_vec.push(match self.v_type {
                1 => {
					MemberItem::new_t1(byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i16() as i32, byte_cursor.read_i16() as i32, byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32())
                },
                2 => {
					MemberItem::new_t2(byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i16() as i32, byte_cursor.read_i16() as i32, byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32())
                },
                3 => {
					MemberItem::new_t3(byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i16() as i32, byte_cursor.read_i16() as i32, byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32(), byte_cursor.read_i32())
                }
                _ => { panic!("self.v_type was wrong number: {}", self.v_type); }
            });
        }
        if self.v_type == 2
        {
            let mut shown = false;
            for _ in 0..self.gap_size
            {
                if byte_cursor.read_byte() != 0
                {
                    #[allow(clippy::collapsible_if)]
                    if !shown
                    {
                        shown = true;
                        println!("This file has content that Heilos would like to see. Please inform him");
                    }
                }
            }
        }

        match self.v_type {
            2 => {
                byte_cursor.read_i32();
            },
            3 => {
                byte_cursor.set_position(num5 as u64);
            }
            _ => {}
        }

        for num8 in 0..num3 {
            if self.v_type == 1 {
                let count = byte_cursor.read_i32();
                for _ in 0..count {
                    self.int_vec.push(IntItem::new(num8 + 1, byte_cursor.read_i32()));
                }
            } else {
                let cat = self.cat_vec.get(num8 as usize).unwrap();
                let count = cat.w + cat.x + cat.y + cat.z;
                for _ in 0..count {
                    self.int_vec.push(IntItem::new(num8 + 1, byte_cursor.read_i32()));
            }
        }

        for _ in 0..num6 {
			self.string_vec.push(StringItem::new(byte_cursor.read_prefixed_string()));
        }
        self.umap.unknown = byte_cursor.read_bytes((num1 - num7) as usize);
        /*
				ReadFile.StringArray = new List<StringItem>();
				for (int num13 = 0; num13 < num6; num13++)
				{
					ReadFile.StringArray.Add(new StringItem(binaryReader.ReadLengthPrefixedString()));
				}
				ReadFile.MapProperties = new UmapItem
				{
					unknown = binaryReader.ReadBytes(num - num7)
				};
			}
		}
		this.ContentArray = new List<ContentItem>();
		for (int num14 = 0; num14 < num3; num14++)
		{
			byte[] array = new byte[ReadFile.CatArray[num14].Length];
			Buffer.BlockCopy(this.Bytes, ReadFile.CatArray[num14].Start, array, 0, array.Length);
			this.ContentArray.Add(new ContentItem(array, ReadFile.CatArray[num14].Type, num14));
		}
		if (Utils.feedback && ReadFile.FunErrCount > 0)
		{
			MessageBox.Show(string.Format("{0} functions failed to read to the end!", ReadFile.FunErrCount));
		}
		if (Utils.feedback && ReadFile.CatErrCount > 0)
		{
			MessageBox.Show(string.Format("Data for {0} of {1} categories failed to load!", ReadFile.CatErrCount, num3));
		}
		if (Utils.feedback && ReadFile.StructErrCount > 0)
		{
			MessageBox.Show(string.Format("{0} Structures failed to read to the end!", ReadFile.StructErrCount));
		}
		if (Utils.feedback && ReadFile.ArrayErrCount > 0)
		{
			MessageBox.Show(string.Format("{0} Arrays failed to load!", ReadFile.ArrayErrCount));
		}
        */
        }
    }

    pub fn read_asset(path: PathBuf) -> Asset {
        let mut file = match std::fs::File::open(&path) {
            Ok(o) => o,
            Err(e) =>
            {
                println!("Failed to find {}", path.display());
                panic!("{}", e);
            }
        };

        let mut asset_buffer = Cursor::new(Vec::new());
        file.read_to_end(asset_buffer.get_mut()).expect("Failed to read file");
        let mut asset = Asset::default();
        let vec = asset_buffer.into_inner();
        println!("found {} bytes", vec.len());
        asset.parse_file_bytes(vec, path);
        asset
    }
}