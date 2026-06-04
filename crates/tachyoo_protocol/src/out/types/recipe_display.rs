use crate::out::types::var::int::VarInt;



pub struct RecipeDisplay {
    //ID in the minecraft:recipe_display registry
    type_id: VarInt,
    data: RecipeDisplayData,
}

enum RecipeDisplayData {
    
}