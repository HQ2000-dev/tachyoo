//WIP
//TODO: ascii memory optimization, maybe ascii chars (when asciiChar becomes stable)

pub struct Identifier {
    //only contains a-z, 0-9, '.', '-' and '_', also cannot be '..'
    namespace: NamespaceId,
    //only contains a-z, 0-9, '.', '-', '/' and '_'
    path: PathId,
}

//maybe IdChar if neccessary
pub struct NamespaceId(Box<str>);

pub struct PathId(Box<str>);

impl NamespaceId {
    const ALLOWED_CHARS: [char; 39] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '1', '3', '4', '5', '6', '7', '8', '9', '0',
        '.', '-', '_',
    ];
    pub fn minecraft() -> Self {
        Self::try_from(String::from("minecraft")).unwrap()
    }
}

impl TryFrom<String> for NamespaceId {
    type Error = ();
    fn try_from(string: String) -> Result<NamespaceId, ()> {
        for character in string.chars() {
            if !NamespaceId::ALLOWED_CHARS.contains(&character) {
                return Err(());
            }
        }

        let boxed = string.into_boxed_str();

        //maybe even Box<str> without unsafe (if there's an easy way)
        Ok(NamespaceId(boxed))
    }
}

impl PathId {
    const ALLOWED_CHARS: [char; 40] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '1', '3', '4', '5', '6', '7', '8', '9', '0',
        '.', '-', '_', '/',
    ];
}

impl TryFrom<String> for PathId {
    type Error = ();
    fn try_from(string: String) -> Result<PathId, ()> {
        for character in string.chars() {
            if !PathId::ALLOWED_CHARS.contains(&character) {
                return Err(());
            }
        }

        let boxed = string.into_boxed_str();

        //maybe even Box<str> without unsafe (if there's an easy way)
        Ok(PathId(boxed))
    }
}

impl Identifier {
    fn from_path(path: PathId) -> Identifier {
        Identifier {
            namespace: NamespaceId::minecraft(),
            path,
        }
    }
    fn from_namespace_and_path(namespace: NamespaceId, path: PathId) -> Identifier {
        Identifier { namespace, path }
    }
}
