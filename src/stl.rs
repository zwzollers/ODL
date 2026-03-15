

pub struct STLTriangle {
    pub normal: [f32; 3],
    pub points: [[f32; 3]; 3],
}

pub struct STL {
    pub triangles: Vec<STLTriangle>
}

#[derive(Debug, Clone)]
pub enum STLFileError {
    InvalidByteCount,
    Unknown,
}

impl STL {
    pub fn try_from_bytes(value: &[u8]) -> Result<Self, STLFileError> {
        if value.len() < 84 {
            return Err(STLFileError::InvalidByteCount);
        }
        let num_triangles = u32::from_le_bytes(value[80..84].try_into().unwrap()) as usize;
        
        let triangles = (0..num_triangles).into_iter().map(|i| {
            let index = 84 + (i * 50);
            STL::get_triangle(value, index)
        }).collect();
        
        Ok( STL { triangles } ) 
    }
    
    pub fn get_triangle(data: &[u8], index: usize) -> STLTriangle {
        STLTriangle { 
            normal: STL::get_vec3(data, index),
            points: [
                STL::get_vec3(data, index+12),
                STL::get_vec3(data, index+24),
                STL::get_vec3(data, index+36),
            ], 
        }
    }
    
    pub fn get_vec3(data: &[u8], index: usize) -> [f32; 3] {
        [
            f32::from_le_bytes(data[index+0..index+4 ].try_into().unwrap()),
            f32::from_le_bytes(data[index+4..index+8 ].try_into().unwrap()),
            f32::from_le_bytes(data[index+8..index+12].try_into().unwrap()),
        ]
    }
}