use cucumber::{given,when, then, World};
use tokio::time::sleep;
use tokio::time::Duration;
#[path = "../src/cat.rs"] mod cat;
use cat::animal;



#[derive(Debug, World)]
#[world(init = Self::new)] 
pub struct AnimalWorld 
{
    animal: animal::Animal,
    cat: cat::Cat,
}
impl AnimalWorld 
{
    fn new() -> Self 
    {
        Self 
        {
            animal: animal::Animal
            {
                hungry: (false) 
            },
            cat: cat::Cat 
            { 
                animal: animal::Animal 
                { 
                    hungry: (false) 
                } 
            }
        }
    }    
}
fn set_world_animal_is_hungry(world: &mut AnimalWorld, animal: String, state: bool) 
{
    match animal.as_str() 
    {
        "animal" => world.animal.hungry = state,
        "cat" => world.cat.animal.hungry = state,
        s => panic!("expected 'animal' or 'cat', found: {s}"),
    }    
}
fn get_world_animal_is_hungry(world: &mut AnimalWorld, animal: String)-> bool
{
    match animal.as_str() 
    {
        "animal" => return world.animal.hungry,
        "cat" => return world.cat.animal.hungry,
        s => panic!("expected 'animal' or 'cat', found: {s}"),
    }    
}
fn feed_world_animal(world: &mut AnimalWorld, animal: String) 
{
    match animal.as_str() 
    {
        "animal" => world.animal.feed(),
        "cat" => world.cat.animal.feed(),
        s => panic!("expected 'animal' or 'cat', found: {s}"),
    }    
}



#[given(expr = "a {word} {word}")]
fn hungry_animal(world: &mut AnimalWorld, hunger: String, animal: String) 
{
    match hunger.as_str() 
    {
        "hungry" => set_world_animal_is_hungry(world, animal, true),
        "satiated" => set_world_animal_is_hungry(world, animal, false),
        s => panic!("expected 'hungry' or 'satiated', found: {s}"),
    }    
}

#[when(expr = "I feed the {word}")]
async fn feed_animal(world: &mut AnimalWorld, animal: String) 
{
    sleep(Duration::from_secs(2)).await;

    feed_world_animal(world, animal);
}

#[then(expr = "the {word} is not hungry")]
fn animal_is_fed(world: &mut AnimalWorld, animal: String) 
{
    assert!(!get_world_animal_is_hungry(world, animal));
}

#[tokio::main]
async fn main() {
    AnimalWorld::run("tests/features").await;
}