use std::vec::Vec;

pub fn get_token_types<'a>() -> Vec<(&'a str, &'a str)>
{
    // TODO: Read types from rules file
    let mut token_types: Vec<(&str, &str)> = Vec::new();

    token_types.push(("comment.line",       	r"//[^\n|$]*"));
    token_types.push(("comment.block",      	r"/\*([^\*]|(\*[^/]))*\*/"));
    token_types.push(("control",            	r"\.|\(|\)|\{|}|\[|]|;|,"));
    token_types.push(("literal.int",        	r"(0(x|b|o|d))?[:digit:]+"));
    token_types.push(("literal.long",       	r"(0(x|b|o|d))?[:digit:]+[lL]"));
    token_types.push(("literal.float",      	r"(\.(\d+)(f|F)?)|((\d+)\.(\d+)(f|F)?)|((\d+)(f|F))"));
    token_types.push(("literal.double",     	r"[:digit:]*\.[:digit:]+"));
    token_types.push(("literal.char",       	r"'[a-z A-Z]?'"));
    token_types.push(("literal.string",     	"\"([^\"\n\r]|\\.)*\""));
    token_types.push(("literal.boolean",    	r"true|false"));
    token_types.push(("literal.null",       	r"null"));
    token_types.push(("block.generics.args",	r"<( )*([a-zA-Z],?( )*)+( )*>"));
    token_types.push(("operator.unary",     	r"(\+\+)|(--)"));
    token_types.push(("operator.binary",    	r"((\+|<<|>>|-|/|\*|\||&)=?)|="));
    token_types.push(("operator.comparator",	r">|>=|<|<=|&&|\|\||==|instanceof"));
    token_types.push(("type",               	r"boolean|long|int|byte|short|char|double|float|void"));
    token_types.push(("mod.permission",     	r"public|private|protected"));
    token_types.push(("mod.access",    			r"static"));
    token_types.push(("mod.behaviour", 			r"final|volitile|transient|synchronized|native|abstract|throws"));
    token_types.push(("mod.inheritance",		r"extends|implements"));
    token_types.push(("action",             	r"return|continue|break|throw|new|assert|strictfp"));
    token_types.push(("construct.conditional", 	r"if|else"));
    token_types.push(("construct.handles",		r"try|catch|finally"));
    token_types.push(("construct.switch", 		r"switch|case:|default:"));
    token_types.push(("construct.loop",		 	r"do|while|for"));
    token_types.push(("construct.type", 		r"class|interface|enum"));
    token_types.push(("special.package",      	r"package"));
    token_types.push(("special.reserved",   	r"goto|const"));
    token_types.push(("special.import",     	r"import( +static)?( )+([a-zA-Z\._])+\*?"));
    token_types.push(("identifier",				r"([:alpha:]|[\$_])+([:alnum:]|[\$_])*"));

    token_types
}
