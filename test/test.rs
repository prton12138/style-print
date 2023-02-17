use style_print::*;
#[test]
fn it_works() {
	style_print!{
		|RED_CHAR,HIGHLIGHT|"error:{}+{}!={}",1,2,{1+2};
		|YELLOW_CHAR,FLASH|"warning:{}",{let mut a = String::from("test");a+=" warning";a};
		|GREEN_BG,BLACK_CHAR,REVERSE|"Compile success";
	}
}
