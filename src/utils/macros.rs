#[macro_export]
macro_rules! set {
{$($i:expr),*}=>{
        {
            let mut _tmp_set = HashSet::new();
            $(
                _tmp_set.insert($i);
            )*
        _tmp_set
        }
    };
}

#[macro_export]
macro_rules! set_list {
[$({$($i:expr),*}),*]=>{{
    let mut _list_of_set = Vec::new();
        $(
        {
        let mut _set_on_list = HashSet::new();
            $(
                _set_on_list.insert($i);
            )*
        _list_of_set.push(_set_on_list);
        }
        )*
    _list_of_set}
    };
}

#[macro_export]
macro_rules! dict {
    ($($k:expr => $v:expr),*)
     => {
        {
        let mut _map = HashMap::new();
        $(
            _map.insert($k,$v);
        )*
       _map
        }
    };
}

#[macro_export]
macro_rules! mat {
    ($([$($x:tt),* $(,)?]),+ $(,)?) => {
        vec![$(
            mat![$($x),*]
        ),+]
    };
    ($($x:expr),* $(,)?) => {
        vec![$($x),*]
    };
}
