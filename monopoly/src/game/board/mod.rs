use uuid::Uuid;


pub struct Board {
    max_size: usize
}

impl Board {
    pub fn new() -> Self {
        Self { max_size: 40 }
    }
}

#[derive(PartialEq)]
pub enum TileType {
    Property,
    Chance,
    CommunityChest,
    Fine,
    GoToJail,
    Go,
    Jail,
    FreeParking
}

pub struct BoardTile {
    id: Uuid,
    section_type: TileType,
}

impl BoardTile {
    pub fn new(id: Uuid, section_type: TileType) -> Self {
        Self { id: id, section_type: section_type }
    }

    pub fn is_property(&self) -> bool {
        return self.section_type == TileType::Property;
    }

    pub fn is_chance(&self) -> bool {
        return self.section_type == TileType::Chance;
    }

    pub fn is_community_chance(&self) -> bool {
        return self.section_type == TileType::CommunityChest;
    }

    pub fn is_free(&self) -> bool {
        return match self.section_type {
            TileType::Go => true,
            TileType::Jail => true,
            TileType::FreeParking => true,
            _ => false,
        };
    }

    pub fn is_go_to_jail(&self) -> bool {
        return self.section_type == TileType::GoToJail;
    }

    pub fn is_fine(&self) -> bool {
        return self.section_type == TileType::Fine;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_section_property() {
        let section = BoardTile::new(Uuid::new_v4(), TileType::Property);

        assert!(section.is_property());
    }
}