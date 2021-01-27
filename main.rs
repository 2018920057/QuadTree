use std::io;

const MAX_SIZE :usize = 16;
 
fn main() {
  let mut decompressed : [[bool;MAX_SIZE];MAX_SIZE] = [[false;MAX_SIZE];MAX_SIZE];
  let mut quad_tree : String = String::new();

  io::stdin().read_line(&mut quad_tree).expect("stdin error");
  
  let mut reverse_tree : String = reverse(&mut quad_tree.clone());
  
  decompress(&mut decompressed, &mut quad_tree, 0, 0, MAX_SIZE);
  print(decompressed);
  
  println!("{}",reverse_tree);
  decompress(&mut decompressed, &mut reverse_tree, 0, 0, MAX_SIZE);
  print(decompressed);
}

//압축 해제하는 함수, str:입력받은 문자열, y,x: 좌표, size:크기
fn decompress(arr : &mut [[bool;MAX_SIZE];MAX_SIZE], stri : &mut String, y : i32, x : i32, size : usize){
  let i32_size = size as i32;
  //한 글자를 지우면서 실행
  match stri.remove(0){
    'x' => {
      decompress(arr, stri, y, x, size/2);
      decompress(arr, stri, y, x+i32_size/2, size/2);
      decompress(arr, stri, y+i32_size/2, x, size/2);
      decompress(arr, stri, y+i32_size/2, x+i32_size/2, size/2);
    },
    'b' => {
      for i in y..y+i32_size{
        for j in x..x+i32_size{
          let ii = i as usize;
          let jj = j as usize;
          arr[jj][ii]=false;
        }
      }},
    'w' => {
      for i in y..y+i32_size{
        for j in x..x+i32_size{
          let ii = i as usize;
          let jj = j as usize;
          arr[jj][ii]=true;
        }
      }
    },
     _ => {println!("error!");},
  }
}

//쿼드 트리를 출력하는 함수
fn print(arr : [[bool;MAX_SIZE];MAX_SIZE]){
  for i in 0..MAX_SIZE{
    for j in 0..MAX_SIZE{
      print!("{}", match arr[j][i]{true => "■ ", false => "□ ",});
    }
    println!();
  }
}

//쿼드 트리를 상하로 뒤집는 함수
fn reverse(stri : &mut String) -> String{
  match stri.remove(0) {
    'x' => {
      let upper_left = reverse(stri);
      let upper_right = reverse(stri);
      let lower_left = reverse(stri);
      let lower_right = reverse(stri);
      return format!("x{}{}{}{}",lower_left,lower_right,upper_left,upper_right);
    },
    'b' => {
      return String::from("b");
    },
    'w' => {
      return String::from("w");
    },
    _ => {
      panic!("error");
    },
  }
}