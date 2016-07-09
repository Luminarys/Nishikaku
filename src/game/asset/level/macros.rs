macro_rules! tget {
    ($t:expr, $k:expr, angle, $s:expr) => {
        match $t.get($k) {
            Some(v) => tangle!(v.clone(), $s),
            _ => return Err(format!("Could find key {:?} in {:?}!", $k, $s))
        }
    };

    ($t:expr, $k:expr, angle, $s:expr, $d:expr) => {
        match $t.get($k) {
            Some(v) => tangle!(v.clone(), $s),
            _ => $d
        }
    };

    ($t:expr, $k:expr, point, $s:expr) => {
        match $t.get($k) {
            Some(ref v) => {
                if !v.as_slice().is_none() {
                    tpoint!(v.clone(), $s)
                } else {
                    return Err(format!("Expected array, got {:?}!", v))
                }
            }
            _ => return Err(format!("Could find key {:?} in {:?}!", $k, $s))
        }
    };

    ($t:expr, $k:expr, num, $s:expr) => {
        match $t.get($k) {
            Some(v) => tint!(v.clone(), $s),
            None => return Err(format!("Could find key {:?} in {:?}!", $k, $s))
        }
    };

    ($t:expr, $k:expr, $vt:path, $s:expr) => {
        match $t.get($k) {
            Some(v) => {
                match v {
                    &$vt(ref r) => r,
                    _ => return Err(format!("Could find key {:?} of correct type in {:?}!", $k, $s))
                }
            }
            None => return Err(format!("Could find key {:?} in {:?}!", $k, $s))
        }
    };

    ($t:expr, $k:expr, num, $s:expr, $d:expr) => {
        match $t.get($k) {
            Some(v) => tint!(v.clone(), $s),
            None => $d
        }
    };
    ($t:expr, $k:expr, $vt:path, $s:expr, $d:expr) => {
        match $t.get($k) {
            Some(v) => {
                match v {
                    &$vt(ref r) => r,
                    _ => return Err(format!("Could find key {:?} of correct type in {:?}!", $k, $s))
                }
            }
            None => $d
        }
    };
}

macro_rules! tint {
    ($v:expr, $s:expr) => {
        match $v.clone() {
            toml::Value::Integer(i) => i as f32,
            toml::Value::Float(f) => f as f32,
            t => return Err(format!("{:?} was expected to be integer or float, found as {:?}", $s, t))
        }
    }
}

macro_rules! tpoint {
    ($v:expr, $s:expr) => {
        match $v.clone() {
            toml::Value::Array(a) => {
                match &a[0] {
                    &toml::Value::String(ref s) => {
                        let x = if a.len() > 1 {
                            tint!(a[1], $s)
                        } else {
                            0.0
                        };
                        let y = if a.len() > 2 {
                            tint!(a[2], $s)
                        } else {
                            0.0
                        };
                        if s == "current" {
                            Point::Current(Vector2::new(x, y))
                        } else if s == "player" {
                            Point::Player(Vector2::new(x, y))
                        } else {
                            return Err(format!("{:?} was expected to be current or player, found as {:?}", $s, s))
                        }
                    }
                    &toml::Value::Integer(_) | &toml::Value::Float(_) => {
                        let x = tint!(a[0], $s);
                        let y = tint!(a[1], $s);
                        Point::Fixed(Vector2::new(x, y))
                    }
                    t => return Err(format!("{:?} was expected to have a first value of 'current', 'player', or a number, found as {:?}", $s, t))
                }
            }
            t => return Err(format!("{:?} was expected to be a point(must use array brackets), found as {:?}", $s, t))
        }
    }
}

macro_rules! tangle {
    ($v:expr, $s:expr) => {
        match $v.clone() {
            toml::Value::String(s) => {
                if s == String::from("player") {
                    Angle::Player(0.0)
                } else {
                    return Err(format!("Angles currently only support player for custom values"))
                }
            }
            toml::Value::Array(a) => {
                match &a[0] {
                    &toml::Value::String(ref s) => {
                        if *s == String::from("player") {
                            Angle::Player(tint!(a[1], $s))
                        } else {
                            return Err(format!("Angles currently only support player for custom values"))
                        }
                    }
                    t => {
                        return Err(format!("{:?} was expected to be a string, number, or array of string/number, found as {:?}", $s, t))
                    }
                }
            }
            toml::Value::Integer(_) | toml::Value::Float(_) => {
                Angle::Fixed(tint!($v, $s))
            }
            t => return Err(format!("{:?} was expected to be a point(must use array brackets), found as {:?}", $s, t))
        }
    }
}

