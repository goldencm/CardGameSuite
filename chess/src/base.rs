


mod chess_piece {
    struct Piece {
        p_type : PieceTypes,
        color : Color
    }

    enum PieceTypes {
        King,
        Queen,
        Bishop,
        Rook,
        Knight,
        Pawn
    }

    enum Color { 
        Black,
        White
    }
}

mod chess_board {

    struct Tile {
        position : (u32, u32),
        mut contains_peice : Option<crate::chess_piece::Piece>,
    }

    impl Tile {
        //Checks to see if tile is empty
        fn is_empty(&self, x : u32, y : u32) -> bool {
            if (self.position.0 == x && self.position.1 == y) 
                return contains_peice.is_none();
            return true;
        }

        //Sets the peice on current tile to none
        fn remove(&self) {
            self.contains_peice = None::<crate::chess_piece::Piece>
        }

        //Sets the peice onto the tile
        fn move(&self, x : u32, y : u32, piece : crate::chess_piece::Piece) {
            if (self.position.0 == x && self.position.1 == y) {
                self.contains_peice = Some(peice);
            }
        }

        //Returns the peice type
        fn get_piece(&self) {
            return self.contains_peice;
        }
    }
}