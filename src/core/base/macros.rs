// Spin RPC library, copyright 2015-2017 Georg Brandl.

#[macro_export]
macro_rules! spin_base_trait {
    (cmds = [$($cname:ident => ($cdoc:expr, $cintype:ident, $couttype:ident, $cfunc:ident)),* $(,)*],
     attrs = [$($aname:ident => ($adoc:expr, $atype:ident, $arfunc:ident, $awfunc:ident)),* $(,)*],
     props = [$($pname:ident => ($pdoc:expr, $ptype:ident, $pdef:expr)),* $(,)*] $(,)*) => {
        #[derive(Default)]
        pub struct Props {
            _descriptions: Vec<$crate::arg::PropDesc>,
            $(
                pub $pname: rust_type_for_data_type!($ptype),
            )*
        }

        pub trait GetProps {
            fn mut_props(&mut self) -> &mut Props;
        }

        pub trait Base : $crate::device::Device + GetProps {
            fn query_cmd_descs() -> Vec<$crate::arg::CmdDesc> {
                vec![$($crate::arg::cmd_info(stringify!($cname), $cdoc,
                                             $crate::arg::DataType::$cintype,
                                             $crate::arg::DataType::$couttype),)*]
            }

            fn query_attr_descs() -> Vec<$crate::arg::AttrDesc> {
                vec![$($crate::arg::attr_info(stringify!($aname), $adoc,
                                              $crate::arg::DataType::$atype),)*]
            }

            fn query_prop_descs(props: &Props) -> Vec<$crate::arg::PropDesc> {
                props._descriptions.clone()
            }

            #[allow(unused_variables)]
            fn exec_cmd(&mut self, cmd: &str, arg: $crate::Value)
                        -> Option<$crate::SpinResult<$crate::Value>> {
                match cmd {
                    $(stringify!($cname) =>
                        match arg.extract() {
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
                        match val.extract() {
                            Ok(val) => Some(self.$awfunc(val)),
                            Err(err) => Some(Err(err)),
                        },
                    )*
                    _ => None
                }
            }

            #[allow(unused_variables, unused_mut)]
            fn init_props(props: &mut Props,
                          cfg_prop_map: &mut ::fnv::FnvHashMap<String, $crate::Value>) {
                $(
                    props._descriptions.push(
                        $crate::arg::prop_info(stringify!($pname), $pdoc,
                                               $crate::arg::DataType::$ptype,
                                               $crate::Value::from($pdef)));
                    props.$pname = $pdef;
                    if let Some(cfg_value) = cfg_prop_map.remove(stringify!($pname)) {
                        if let Some(value) = cfg_value.convert($crate::arg::DataType::$ptype) {
                            props.$pname = value.extract().unwrap();
                            debug!("property {} from config: {:?}",
                                   stringify!($pname), props.$pname);
                        } else {
                            warn!("XXX property conversion failure");
                            debug!("property {} from default: {:?}",
                                   stringify!($pname), props.$pname);
                        }
                    } else {
                        debug!("property {} from default: {:?}",
                               stringify!($pname), props.$pname);
                    }
                )*
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
                        if let Some(val) = val.convert($crate::arg::DataType::$ptype) {
                            props.$pname = val.extract().unwrap();
                            return Some(Ok(()));
                        } else {
                            return Some(spin_err!(
                                $crate::error::ARG_ERROR,
                                &format!("Wrong property type, expected {:?}",
                                         $crate::arg::DataType::$ptype)));
                        }
                    }
                )*;
                None
            }

            $(
                fn $cfunc(&mut self, val: rust_type_for_data_type!($cintype))
                          -> $crate::SpinResult<rust_type_for_data_type!($couttype)> {
                    self::default::$cfunc(self, val)
                }
            )*

            $(
                fn $arfunc(&mut self) -> $crate::SpinResult<rust_type_for_data_type!($atype)> {
                    self::default::$arfunc(self)
                }
            )*

            $(
                fn $awfunc(&mut self, val: rust_type_for_data_type!($atype)) -> $crate::SpinResult<()> {
                    self::default::$awfunc(self, val)
                }
            )*
        }
    }
}
