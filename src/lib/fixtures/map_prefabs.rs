pub struct Prefab<'a> {
    pub structure_str: &'a str,
    pub width: i32,
    pub height: i32,
}

pub const FORTRESS: Prefab = Prefab {
    structure_str: "
    ------------
    ---######---
    ---#----#---
    ---#----#---
    ---#----#---
    -###----###-
    -#--------#-
    -#--------#-
    -M--------M-
    -#--------#-
    -#--------#-
    -###----###-
    ---#----#---
    ---#----#---
    ---#----#---
    ---######---
    ------------",
    width: 12,
    height: 16,
};

pub const CHESS: Prefab = Prefab {
    structure_str: "
    #-#-#-#-#-#
    -#-#-#-#-#-
    #-#-#-#-#-#
    -#-#-#-#-#-
    #-#-#-#-#-#
    ",
    width: 11,
    height: 5,
};
