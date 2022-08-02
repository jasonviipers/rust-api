table! {
    admin(id) {
        id ->Nullable <Integer>,
        username -> Text,
        password -> Text,
    }
}

table! {
    host(id) {
        id ->Nullable <Integer>,
        name -> Text,
        username -> Text,
        password -> Text,
    }
}

table! {
    privacy(id) {
        id ->Nullable <Integer>,
        body -> Text,
    }
}