use std::collections::HashMap;

pub fn replies() -> HashMap<String, String> {
    let mut out: HashMap<String, String> = HashMap::new();

    out.insert("xd".to_string(), "XDDDDD".to_string());
    out.insert("basiert".to_string(), "> basiert\n\nbasiert auf was?".to_string());
    out.insert("fokus".to_string(), "> Fokus\n\nIch hab in den Fokus gekaggert 😎".to_string());
    out.insert("focus".to_string(), "> Focus\n\nIch hab in den Fokus gekaggert 😎".to_string());
    out.insert("sus".to_string(), "ඞ".to_string());
    out.insert("das ist doch der".to_string(), "Ich war nicht derjenige".to_string());
    out.insert("shisha".to_string(), "Merkel mach Shisha auf 😡".to_string());
    out.insert("linux".to_string(), "> linux\n\nIch benutze Bogen bei dem Weg".to_string());
    out.insert("Arch".to_string(), "> linux\n\nIch benutze Bogen bei dem Weg".to_string());
    out.insert("Bogen".to_string(), "> linux\n\nIch benutze Bogen bei dem Weg".to_string());

    out
}
