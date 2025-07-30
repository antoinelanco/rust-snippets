use std::any::Any;

use num_enum::TryFromPrimitive;
use rust_snippets::{DataType, E1, E2, PropId, get_data, get_type};

/// On veut récupérer la valeur d'une propriété via le SDK.
/// Pour cela, on commence par obtenir son type, puis on récupère
/// le pointeur vers la donnée correspondante.
///
/// Ce que je trouve stylé, c'est que le type vers lequel le pointeur
/// va être casté est inféré automatiquement.
pub fn get_prop<T, U>(prop_id: PropId) -> U
where
    U: TryFromPrimitive<Primitive = T> + Default,
    T: 'static,
{
    let data_type = get_type(&prop_id);
    let ptr = get_data(&prop_id);
    let boxed: Box<dyn Any> = match data_type {
        DataType::I32 => Box::new(ptr as i32),
        DataType::U32 => Box::new(ptr as u32),
    };
    let unboxed = boxed.downcast::<T>().expect("Type mismatch");
    U::try_from_primitive(*unboxed).unwrap_or_default()
}

#[test]
/// L’annotation de type des enums `E1` et `E2` aux lignes 96 et 98 définit `U` dans `get_prop`,
/// et `U` permet ensuite de définir `T`. Comme les enums `E1` et `E2` ont un type associé,
/// la fonction `get_prop` sait comment faire la conversion.
///
/// Ce que je trouve cool, c’est que j’ai réussi à créer une fonction qui récupère dynamiquement
/// la valeur d’une propriété, tout en profitant du typage statique. Si le type annoté
/// de l’enum correspond à celui attendu par le SDK, on profite pleinement de l’analyse statique.
fn dyn_cast() {
    let e1: E1 = get_prop(PropId::PropE1);
    println!("E1 : {e1:?}");
    let e2: E2 = get_prop(PropId::PropE2);
    println!("E2 : {e2:?}");
}
