use std::collections::HashMap;

struct DataTypeMeta {
    member_map: HashMap<&'static str, DataType>,
}

impl DataTypeMeta {
    fn new() -> Self {
        DataTypeMeta {
            member_map: HashMap::new(),
        }
    }

    fn contains(&self, item: &str) -> bool {
        self.member_map.contains_key(item)
    }

    fn get(&self, item: &str) -> &DataType {
        self.member_map.get(item).unwrap()
    }

    fn iter(&self) -> std::collections::hash_map::Keys<'_, &'static str, DataType> {
        self.member_map.keys()
    }

    fn len(&self) -> usize {
        self.member_map.len()
    }

    fn from_name(&self, name: &str) -> Option<&DataType> {
        self.member_map.get(name)
    }

    fn from_type(&self, python_type: &'static str) -> Option<&DataType> {
        match python_type {
            "list" | "range" | "tuple" => Some(&DataType::ARRAY),
            "bool" => Some(&DataType::BOOLEAN),
            "datetime.date" | "datetime.datetime" => Some(&DataType::DATETIME),
            "datetime.timedelta" => Some(&DataType::TIMEDELTA),
            "decimal.Decimal" | "float" | "int" => Some(&DataType::FLOAT),
            "dict" => Some(&DataType::MAPPING),
            "NoneType" => Some(&DataType::NULL),
            "set" => Some(&DataType::SET),
            "str" => Some(&DataType::STRING),
            "_PYTHON_FUNCTION_TYPE" => Some(&DataType::FUNCTION),
            _ => None,
        }
    }

    fn from_value(&self, python_value: &dyn std::any::Any) -> Option<&DataType> {
        if python_value.is::<bool>() {
            return Some(&DataType::BOOLEAN);
        } else if python_value.is::<chrono::NaiveDateTime>() {
            return Some(&DataType::DATETIME);
        } else if python_value.is::<chrono::Duration>() {
            return Some(&DataType::TIMEDELTA);
        } else if python_value.is::<f64>() || python_value.is::<i64>() {
            return Some(&DataType::FLOAT);
        } else if python_value.is::<std::collections::HashMap<_, _>>() {
            return Some(&DataType::MAPPING);
        } else if python_value.is::<std::collections::HashSet<_>>() {
            return Some(&DataType::SET);
        } else if python_value.is::<String>() {
            return Some(&DataType::STRING);
        } else if python_value.is::<fn()>() {
            return Some(&DataType::FUNCTION);
        }
        None
    }

    fn is_compatible(&self, dt1: &DataType, dt2: &DataType) -> bool {
        match (dt1, dt2) {
            (DataType::UNDEFINED, _) | (_, DataType::UNDEFINED) => true,
            (DataType::BOOLEAN, DataType::BOOLEAN)
            | (DataType::DATETIME, DataType::DATETIME)
            | (DataType::TIMEDELTA, DataType::TIMEDELTA)
            | (DataType::FLOAT, DataType::FLOAT)
            | (DataType::NULL, DataType::NULL)
            | (DataType::STRING, DataType::STRING) => true,
            (DataType::FUNCTION, DataType::FUNCTION) => {
                // Assuming return_type, argument_types, minimum_arguments exist in DataType enum
                // Replace these with actual fields.
                dt1.return_type == dt2.return_type
                    && dt1.argument_types == dt2.argument_types
                    && dt1.minimum_arguments == dt2.minimum_arguments
            }
            (DataType::ARRAY(val_type1), DataType::ARRAY(val_type2))
            | (DataType::MAPPING(_, val_type1), DataType::MAPPING(_, val_type2)) // TODO: Do we need to validate the key type here?
            | (DataType::SET(val_type1), DataType::SET(val_type2)) => {
                self.is_compatible(val_type1, val_type2) && self.is_compatible(val_type1, val_type2)
            }
            _ => false,
        }
    }
}

pub enum DataType {
    ARRAY(Box<DataType>),
    BOOLEAN,
    DATETIME,
    FLOAT,
    FUNCTION,
    MAPPING(Box<DataType>, Box<DataType>),
    NULL,
    SET(Box<DataType>),
    STRING,
    TIMEDELTA,
    UNDEFINED,
}

struct _DataTypeDef;

impl _DataTypeDef {
    fn new() -> Self {
        _DataTypeDef
    }
}

impl std::cmp::PartialEq for _DataTypeDef {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

// Define each DataType as a constant
const ARRAY: DataType = DataType::ARRAY(Box::new(DataType::UNDEFINED));
const BOOLEAN: DataType = DataType::BOOLEAN;
const DATETIME: DataType = DataType::DATETIME;
const FLOAT: DataType = DataType::FLOAT;
const FUNCTION: DataType = DataType::FUNCTION;
const MAPPING: DataType = DataType::MAPPING(Box::new(DataType::UNDEFINED), Box::new(DataType::UNDEFINED));
const NULL: DataType = DataType::NULL;
const SET: DataType = DataType::SET(Box::new(DataType::UNDEFINED));
const STRING: DataType = DataType::STRING;
