#[derive(Clone, Debug)]
pub struct SvgSystem {
    pub label: String,
    pub level: i32,
    pub parent: String,
    pub children: Vec<SvgSystem>,
}

#[derive(Clone, Debug, Default)]
pub struct SystemNode {
    pub label: String,
    pub level: i32,
    pub x: f64,
}

impl SystemNode {
    pub fn get_node_y(&self) -> f64 {
        let min = 100.0;
        let previous_levels_height = (1..=self.level as u64).sum::<u64>() * 5 / 2;
        min + previous_levels_height as f64 + self.level as f64 * 70.0
    }
    pub fn get_node_width(&self) -> f64 {
        match self.level {
            0 => 90.0,
            1 => 75.0,
            2 => 60.0,
            3 => 45.0,
            4 => 35.0,
            _ => 25.0,
        }
    }

    pub fn get_node_height(&self) -> f64 {
        match self.level {
            0 => 35.0,
            1 => 30.0,
            2 => 25.0,
            3 => 20.0,
            4 => 15.0,
            _ => 10.0,
        }
    }

    pub fn get_node_font_size(&self) -> &'static str {
        match self.level {
            0 => "1.1rem",
            1 => "0.9rem",
            2 => "0.8rem",
            3 => "0.7rem",
            4 => "0.6rem",
            _ => "0.5rem",
        }
    }
}
