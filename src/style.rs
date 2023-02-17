use syn::PatIdent;
macro_rules! style_def {
	{$style:expr, $name:ident, $uname:ident} => {
		(stringify!($uname),$style)
	};
}
pub static STYLE_IDENT_MAP:[(&str, &str); 25] = [
	//字体颜色
	style_def!{"30" , black_char, BLACK_CHAR},
	style_def!{"31" , red_char, RED_CHAR},
	style_def!{"32" , green_char, GREEN_CHAR},
	style_def!{"33" , yellow_char, YELLOW_CHAR},
	style_def!{"34" , blue_char, BLUE_CHAR},
	style_def!{"35" , purple_char, PURPLE_CHAR},
	style_def!{"36" , deep_green_char, DEEP_GREEN_CHAR},
	style_def!{"37" , white_char, WHITE_CHAR},

	//背景颜色
	style_def!{"40" , back_bg, BLACK_BG},
	style_def!{"41" , red_bg, RED_BG},
	style_def!{"42" , green_bg, GREEN_BG},
	style_def!{"43" , yellow_bg, YELLOW_BG},
	style_def!{"44" , blue_bg, BLUE_BG},
	style_def!{"45" , purple_bg, PURPLE_BG},
	style_def!{"46" , deep_green_bg, DEEP_GREEN_BG},
	style_def!{"47" , white_bg, WHITE_BG},

	//字体特效
	style_def!{"0" , none, NONE},
	style_def!{"1" , highlight, HIGHLIGHT},
	style_def!{"3" , italic, ITALIC},
	style_def!{"4" , underline, UNDERLINE},
	style_def!{"5" , flash, FLASH},
	style_def!{"7" , reverse, REVERSE},
	style_def!{"8" , fade, FADE},
	style_def!{"9" , midline, MIDLINE},
	style_def!{"21" , double_underline, DOUBLE_UNDERLINE}
];
pub fn styles_find(style_ident:&PatIdent) -> syn::Result<String> {
	let style_name = style_ident.ident.to_string();
	for i in STYLE_IDENT_MAP {
		if style_name == i.0 {
			return Ok(i.1.to_string());
		}
	}
	Err(syn::Error::new(style_ident.ident.span(),"no such style".to_string()))
}
