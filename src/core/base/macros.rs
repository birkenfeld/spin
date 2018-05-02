// Spin RPC library, copyright 2015-2018 Georg Brandl.

#[macro_export]
macro_rules! spin_base_trait {
    (cmds = [$($cname:ident => ($cdoc:expr, $cintype:ty, $couttype:ty, $cfunc:ident)),* $(,)*],
     attrs = [$($aname:ident => ($adoc:expr, $atype:ty, $arfunc:ident, $awfunc:ident)),* $(,)*],
     props = [$($pname:ident => ($pdoc:expr, $ptype:ty, $pdefault:expr)),* $(,)*] $(,)*) => {
        #[derive(Default)]
        pub struct Props {
            _descriptions: Vec<$crate::arg::PropDesc>,
            $(
                pub $pname: <$ptype as $crate::validate::CanValidate>::Base,
            )*
        }

        pub trait GetProps {
            fn mut_props(&mut self) -> &mut Props;
        }

        pub trait Base : $crate::device::Device + GetProps {
            fn query_cmd_descs() -> Vec<$crate::arg::CmdDesc> {
                vec![$($crate::arg::cmd_info(stringify!($cname), $cdoc,
                                             _data_type_!($cintype),
                                             _data_type_!($couttype)),)*]
            }

            fn query_attr_descs() -> Vec<$crate::arg::AttrDesc> {
                vec![$($crate::arg::attr_info(stringify!($aname), $adoc,
                                              _data_type_!($atype)),)*]
            }

            fn query_prop_descs(props: &Props) -> Vec<$crate::arg::PropDesc> {
                props._descriptions.clone()
            }

            #[allow(unused_variables)]
            fn exec_cmd(&mut self, cmd: &str, arg: $crate::Value)
                        -> Option<$crate::SpinResult<$crate::Value>> {
                match cmd {
                    $(stringify!($cname) =>
                        match <$cintype as $crate::validate::CanValidate>::validate(arg) {
                            Ok(arg) => Some(self.$cfunc(arg).map($crate::Value::from)),
                            Err(err) => Some(Err(err)),
                        },
                    )*
                    _ => None
                }
            }

            #[allow(unused_variables)]
            fn read_attr(&mut self, attr: &str) -> Option<$crate::SpinResult<$crate::Value>> {
                match attr {
                    $(stringify!($aname) => Some(self.$arfunc().map($crate::Value::from)),)*
                    _ => None
                }
            }

            #[allow(unused_variables)]
            fn write_attr(&mut self, attr: &str, val: $crate::Value) -> Option<$crate::SpinResult<()>> {
                match attr {
                    $(stringify!($aname) =>
                        match <$atype as $crate::validate::CanValidate>::validate(val) {
                            Ok(val) => Some(self.$awfunc(val)),
                            Err(err) => Some(Err(err)),
                        },
                    )*
                    _ => None
                }
            }

            #[allow(unused_variables, unused_mut)]
            fn init_props(props: &mut Props, cfg_prop_map: &mut ::fxhash::FxHashMap<String, $crate::Value>)
                -> $crate::SpinResult<()> {
                $(
                    props._descriptions.push(
                        $crate::arg::prop_info(stringify!($pname), $pdoc,
                                               _data_type_!($ptype),
                                               $crate::Value::from($pdefault)));
                    if let Some(cfg_value) = cfg_prop_map.remove(stringify!($pname)) {
                        if let Some(value) = cfg_value.convert(_data_type_!($ptype)) {
                            props.$pname = <$ptype as $crate::validate::CanValidate>::validate(value)?;
                            debug!("property {} from config: {:?}",
                                   stringify!($pname), props.$pname);
                        } else {
                            return spin_err!($crate::error::CONFIG_ERROR,
                                             &format!("Wrong configured type for {}, expected {:?}",
                                                      stringify!($pname), _data_type_!($ptype)));
                        }
                    } else {
                        props.$pname = $crate::validate::IntoDefault::into_default(&$pdefault)?.extract()?;
                        debug!("property {} from default: {:?}",
                               stringify!($pname), props.$pname);
                    }
                )*
                Ok(())
            }

            #[allow(unused_variables)]
            fn get_prop(props: &Props, prop: &str) -> Option<$crate::SpinResult<$crate::Value>> {
                $(
                    if prop == stringify!($pname) {
                        return Some(Ok($crate::Value::from(props.$pname.clone())));
                    }
                )*;
                None
            }

            #[allow(unused_variables)]
            fn set_prop(props: &mut Props, prop: &str, val: $crate::Value) -> Option<$crate::SpinResult<()>> {
                $(
                    if prop == stringify!($pname) {
                        if let Some(val) = val.convert(_data_type_!($ptype)) {
                            match <$ptype as $crate::validate::CanValidate>::validate(val) {
                                Ok(v) => props.$pname = v,
                                Err(e) => warn!("XXX property validation failure"),
                            }
                            return Some(Ok(()));
                        } else {
                            return Some(spin_err!(
                                $crate::error::ARG_ERROR,
                                &format!("Wrong property type, expected {:?}",
                                         _data_type_!($ptype)))
                            );
                        }
                    }
                )*;
                None
            }

            $(
                fn $cfunc(&mut self, val: _rust_type_!($cintype))
                          -> $crate::SpinResult<_rust_type_!($couttype)> {
                    self::default::$cfunc(self, val)
                }
            )*

            $(
                fn $arfunc(&mut self) -> $crate::SpinResult<_rust_type_!($atype)> {
                    self::default::$arfunc(self)
                }
            )*

            $(
                fn $awfunc(&mut self, val: _rust_type_!($atype)) -> $crate::SpinResult<()> {
                    self::default::$awfunc(self, val)
                }
            )*
        }
    }
}
