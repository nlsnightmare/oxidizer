
#[macro_export]
macro_rules! route {
    ($method:ident, $url:expr, $function:ident) => {
        {
            use oxidizer::router::Route;
            use oxidizer::response::Response;
            use oxidizer::request::Request;

            let action_string = String::from(stringify!($function));
            let route = Route::$method($url, action_string, move |request: Request| {
                let result = $function(request);
                Ok(result)
            });

            route
        }
    }
}

#[macro_export]
macro_rules! get {
    ($url:expr, $function:ident) => { oxidizer::route!(get, $url, $function) }
}
// #[macro_export]
// macro_rules! post {
//     ($url:expr, $con:ty => $function:ident) => { oxidizer::route!(post, $url, $con => $function) }
// }
// #[macro_export]
// macro_rules! put {
//     ($url:expr, $con:ty => $function:ident) => { oxidizer::route!(put, $url, $con => $function) }
// }
// #[macro_export]
// macro_rules! patch {
//     ($url:expr, $con:ty => $function:ident) => { oxidizer::route!(patch, $url, $con => $function) }
// }
// #[macro_export]
// macro_rules! delete {
//     ($url:expr, $con:ty => $function:ident) => { oxidizer::route!(delete, $url, $con => $function) }
// }

// #[macro_export]
// macro_rules! resource {
//     ($url:expr, $con:ty) => {
//         {
//             let single_resource = oxidizer::util::resource_route($url);
//             let single_url = &format!("{}/<{}>", $url, single_resource);

//             let index  = oxidizer::get!($url,          $con => index);
//             let show   = oxidizer::get!(single_url,    $con => show);
//             let store  = oxidizer::post!($url,         $con => store);
//             let put    = oxidizer::put!(single_url,    $con => update);
//             let patch  = oxidizer::patch!(single_url,  $con => update);
//             let delete = oxidizer::delete!(single_url, $con => destroy);

//             vec![index, show, store, put, patch, delete]
//         }
//     };
// }