
/// Combine a namespace and a body name to create a new namespace name.
/// For instance, "OuterClass" and "InnerClass" can be concatenated to obtain the complete namespace of InnerClass
/// @return the concatenated namespace
pub fn concatenate_namespace(namespace: &str, extension: &str) -> String
{
	let mut new_namespace = namespace.to_string();
	new_namespace.push_str(".");
	new_namespace.push_str(extension);

    return new_namespace;
}

/// Extend a label with a new word
/// @return the concatenated label
pub fn concatenate_label(label: &str, extension: &str) -> String
{
	let mut new_label = label.to_string();
	new_label.push_str("_");
	new_label.push_str(extension);

    return new_label;
}

/// Extend a label with a new word
/// @return the concatenated label
pub fn create_label_from_namespace(namespace: &str) -> String
{
    return namespace.to_string().replace(".", "_");
}
