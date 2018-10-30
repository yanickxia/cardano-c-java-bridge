/// Expose the JNI interface for android below

#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use cardano::{address::ExtendedAddr, util::base58};
    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};

    #[no_mangle]
    pub unsafe extern fn Java_daocloud_io_myapplication_RustGreetings_greeting(env: JNIEnv, _: JClass, input: JString) -> jstring {
        
         // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let input: String =
        env.get_string(input).expect("Couldn't get java string!").into();

    if let Ok(address_raw) = base58::decode_bytes(input.as_bytes()) {
        if let Ok(_) = ExtendedAddr::from_bytes(&address_raw[..]) {
            env.new_string("OK").expect("Couldn't create java string!").into_inner()
        } else {
           env.new_string("FAIL").expect("Couldn't create java string!").into_inner()
        }
    } else {
       env.new_string("FAIL").expect("Couldn't create java string!").into_inner()
    }
    }
}