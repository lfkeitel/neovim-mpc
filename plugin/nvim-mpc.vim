if exists('g:loaded_nvim_mpc')
  finish
endif
let g:loaded_nvim_mpc = 1

" Initialize the channel for nvim-mpc
if !exists('s:mpcjobid')
  let s:mpcjobid = 0
endif

" Path to the binary
let s:scriptdir = resolve(expand('<sfile>:p:h') . '/..')
let s:bin = s:scriptdir . '/target/release/neovim-mpc'

" RPC message constants
let s:CurrentSong = 'current_song'
let s:Toggle = 'toggle'
let s:Play = 'play'
let s:Pause = 'pause'
let s:Next = 'next'
let s:Previous = 'previous'

" Entry point
function! s:init()
  call s:connect()
endfunction

" Get the Job ID and check for errors. If no errors, attach RPC handlers to
" the commands.
function! s:connect()
  let jobID = s:GetJobID()

  if 0 == jobID
    echoerr "mpc: cannot start rpc process"
  elseif -1 == jobID
    echoerr "mpc: rpc process is not executable"
  else
    let s:mpcjobid = jobID
    call s:AttachRPCHandlers()
  endif
endfunction

" Function reference in case of RPC start errors
function! s:OnStderr(id, data, event) dict
  echom 'stderr: ' . a:event . join(a:data, "\n")
endfunction

" Start the RPC job and return the job  (channel) ID
function! s:GetJobID()
  if s:mpcjobid == 0
    let jobid = jobstart([s:bin], { 'rpc': v:true, 'on_stderr': function('s:OnStderr') })
    return jobid
  else
    return s:mpcjobid
  endif
endfunction

" Associate commands with their RPC invocations
function! s:AttachRPCHandlers()
  command! -nargs=0 MpcCurrentSong :call s:rpc(s:CurrentSong)
  command! -nargs=0 MpcToggle :call s:rpc(s:Toggle)
  command! -nargs=0 MpcPlay :call s:rpc(s:Play)
  command! -nargs=0 MpcPause :call s:rpc(s:Pause)
  command! -nargs=0 MpcNext :call s:rpc(s:Next)
  command! -nargs=0 MpcPrevious :call s:rpc(s:Previous)
endfunction

" Send an RPC message to the remote process.
function! s:rpc(rpcMessage)
  call rpcnotify(s:mpcjobid, a:rpcMessage)
endfunction

call s:init()
