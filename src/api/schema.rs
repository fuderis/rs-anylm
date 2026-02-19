use crate::prelude::*;

/// The JSON-schema kind
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SchemaKind {
    Object,
    Array,
    String,
    Number,
    Integer,
    Boolean,
    Null,
}

impl Default for SchemaKind {
    fn default() -> Self {
        Self::Object
    }
}

/// The JSON-schema property
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Schema {
    /// The schema type
    #[serde(rename = "type")]
    pub kind: SchemaKind,
    /// The schema description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The string value variants
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "enum")]
    pub variants: Vec<String>,
    /// The minimum value for number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    /// The maximum value for number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    /// The array items type
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_schema: Option<Box<Schema>>,
    /// The object properties
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, Box<Schema>>,
    /// The required object properties
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<bool>,
}

impl Schema {
    /// Creates an object schema
    pub fn object(descr: impl Into<String>) -> Self {
        Self {
            kind: SchemaKind::Object,
            description: match descr.into() {
                s if !s.is_empty() => Some(s),
                _ => None,
            },
            additional_properties: Some(false),
            ..Default::default()
        }
    }

    /// Creates an array schema
    pub fn array(descr: impl Into<String>) -> Self {
        Self {
            kind: SchemaKind::Array,
            description: match descr.into() {
                s if !s.is_empty() => Some(s),
                _ => None,
            },
            ..Default::default()
        }
    }

    /// Creates a string schema
    pub fn string(descr: impl Into<String>) -> Self {
        Self {
            kind: SchemaKind::String,
            description: match descr.into() {
                s if !s.is_empty() => Some(s),
                _ => None,
            },
            ..Default::default()
        }
    }

    /// Creates a number schema
    pub fn number(descr: impl Into<String>) -> Self {
        Self {
            kind: SchemaKind::Number,
            description: match descr.into() {
                s if !s.is_empty() => Some(s),
                _ => None,
            },
            ..Default::default()
        }
    }

    /// Creates a integer schema
    pub fn integer(descr: impl Into<String>) -> Self {
        Self {
            kind: SchemaKind::Integer,
            description: match descr.into() {
                s if !s.is_empty() => Some(s),
                _ => None,
            },
            ..Default::default()
        }
    }

    /// Creates a boolean schema
    pub fn boolean(descr: impl Into<String>) -> Self {
        Self {
            kind: SchemaKind::Boolean,
            description: match descr.into() {
                s if !s.is_empty() => Some(s),
                _ => None,
            },
            ..Default::default()
        }
    }

    /// Creates a null schema
    pub fn null(descr: impl Into<String>) -> Self {
        Self {
            kind: SchemaKind::Null,
            description: match descr.into() {
                s if !s.is_empty() => Some(s),
                _ => None,
            },
            ..Default::default()
        }
    }

    /// Sets string value variants
    pub fn description(mut self, descr: impl Into<String>) -> Self {
        self.description.replace(descr.into());
        self
    }
    /// Sets string value variants
    pub fn set_description(&mut self, descr: impl Into<String>) {
        self.description.replace(descr.into());
    }

    /// Sets string value variants
    pub fn variants(mut self, vars: Vec<String>) -> Self {
        self.variants = vars;
        self
    }
    /// Sets string value variants
    pub fn set_variants(&mut self, vars: Vec<String>) {
        self.variants = vars;
    }

    /// Sets the minimum number value
    pub fn minimum(mut self, min: f64) -> Self {
        self.minimum.replace(min);
        self
    }
    /// Sets the minimum number value
    pub fn set_minimum(&mut self, min: f64) {
        self.minimum.replace(min);
    }

    /// Sets the maximum number value
    pub fn maximum(mut self, max: f64) -> Self {
        self.maximum.replace(max);
        self
    }
    /// Sets the maximum number value
    pub fn set_maximum(&mut self, max: f64) {
        self.maximum.replace(max);
    }

    /// Sets the array items schema
    pub fn items_schema(mut self, schema: Schema) -> Self {
        self.items_schema.replace(Box::new(schema));
        self
    }
    /// Sets the array items schema
    pub fn set_items_schema(&mut self, schema: Schema) {
        self.items_schema.replace(Box::new(schema));
    }

    /// Adds the object properties schema
    pub fn properties(mut self, props: HashMap<impl Into<String>, Box<Schema>>) -> Self {
        self.properties.extend(
            props
                .into_iter()
                .map(|(k, v)| (k.into(), v))
                .collect::<HashMap<_, _>>(),
        );
        self
    }
    /// Adds the object properties schema
    pub fn set_properties(&mut self, props: HashMap<impl Into<String>, Box<Schema>>) {
        self.properties.extend(
            props
                .into_iter()
                .map(|(k, v)| (k.into(), v))
                .collect::<HashMap<_, _>>(),
        );
    }

    /// Adds the object property schema
    pub fn property(mut self, name: impl Into<String>, schema: Schema, required: bool) -> Self {
        let name = name.into();
        if required && !self.required.contains(&name) {
            self.required.push(name.clone());
        }
        self.properties.insert(name, Box::new(schema));
        self
    }
    /// Adds the object property schema
    pub fn set_property(&mut self, name: impl Into<String>, schema: Schema, required: bool) {
        let name = name.into();
        if required && !self.required.contains(&name) {
            self.required.push(name.clone());
        }
        self.properties.insert(name, Box::new(schema));
    }

    /// Adds the required property schema
    pub fn required_property(self, name: impl Into<String>, schema: Schema) -> Self {
        self.property(name, schema, true)
    }
    /// Adds the required property schema
    pub fn set_required_property(&mut self, name: impl Into<String>, schema: Schema) {
        self.set_property(name, schema, true);
    }

    /// Adds the optional property schema
    pub fn optional_property(self, name: impl Into<String>, schema: Schema) -> Self {
        self.property(name, schema, false)
    }
    /// Adds the optional property schema
    pub fn set_optional_property(&mut self, name: impl Into<String>, schema: Schema) {
        self.set_property(name, schema, false);
    }

    /// Adds the required object properties
    pub fn requires(mut self, reqs: Vec<impl Into<String>>) -> Self {
        self.required.extend(reqs.into_iter().map(Into::into));
        self
    }
    /// Adds the required object properties
    pub fn set_requires(&mut self, reqs: Vec<impl Into<String>>) {
        self.required.extend(reqs.into_iter().map(Into::into));
    }

    /// Adds the required object property
    pub fn require(mut self, req: impl Into<String>) -> Self {
        self.required.push(req.into());
        self
    }
    /// Adds the required object property
    pub fn set_require(&mut self, req: impl Into<String>) {
        self.required.push(req.into());
    }
}
