<?php

include("hangeul_keyboard.php");



$str="";

for($x = 1; $x < count($argv); $x++ )
{
  $str.=$argv[$x];
  $str.=" ";
}

$str=Hangeul_Keyboard::convert($str);
echo $str;

?>
