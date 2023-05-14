macro_rules! map {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = BTreeMap::new();
            $(
                map.insert($key, $value);
            )+
            map
        }
    };
}

pub (crate) use map;