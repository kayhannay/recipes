use rocket_contrib::templates::Template;
use model::RecipeName;
use model::Recipe;
use database::get_recipes;
use database::get_recipe;
use database::RecipeDatabase;

#[get("/")]
pub fn recipe_list(connection: RecipeDatabase) -> Template {
    let recipe_list = get_recipes(connection);
    let model = RecipeOverviewModel { recipe_names: recipe_list };
    Template::render("index", &model)
}

#[get("/recipe/<id>")]
pub fn recipe(id: i32, connection: RecipeDatabase) -> Template {
    let mut recipe= get_recipe(id, connection);
    recipe = convert_newline(recipe);
    let model = RecipeModel { recipe };
    Template::render("recipe", &model)
}

#[derive(Serialize)]
struct RecipeOverviewModel {
    recipe_names: Vec<RecipeName>,
}

#[derive(Serialize)]
struct RecipeModel {
    recipe: Recipe,
}

fn convert_newline(mut recipe: Recipe) -> Recipe {
    recipe.ingredients = recipe.ingredients.replace("\n", "<br />");
    recipe.preparation = recipe.preparation.replace("\n", "<br />");
    return recipe;
}
