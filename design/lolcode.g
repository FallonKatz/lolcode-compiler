grammar lolcode;

HAI 	:	'#HAI';
KTHXBYE	:	'#KTHXBYE';
TEXT	:	('A'..'Z' | 'a'..'z' | '0'..'9' | ',' | '.' | '"' | ':' | '?' | '!' | '%' | '/' )+;
URL	:	('A'..'Z' | 'a'..'z' | '0'..'9' | '/' | ':' | '.' | '?' | '!' | '%' | '&' | '=' | '+' | ',' | '-' | '#')+;

document 
	:	HAI ' ' body ' ' KTHXBYE;
body 	:	(comment | head | content)+;
comment	:	'#OBTW' ' ' TEXT ' ' '#TLDR';
head 	:	'#MAEK HEAD' '#GIMMEH TITLE' TEXT '#MKAY' '#OIC';
content	:	' ' | paragraph | bold | italics | list | sound | video | var_def | var_use;
paragraph
	:	'#MAEK PARAGRAF' content* '#OIC';
bold	:	'#GIMMEH BOLD' TEXT '#MKAY';
italics	:	'#GIMMEH ITALICS' TEXT '#MKAY';
list	:	'#MAEK LIST' item* '#OIC';
item	:	'#GIMMEH ITEM' content* '#MKAY';
sound	:	'#GIMMEH SOUNDZ' URL '#MKAY';
video	:	'#GIMMEH VIDZ' URL '#MKAY';
var_def	:	'#I HAZ' TEXT '#IT IZ' TEXT '#MKAY';
var_use	:	'#LEMME SEE' TEXT '#MKAY';
