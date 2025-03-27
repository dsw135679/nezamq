#[allow(unused_macros)]
macro_rules! generate_placement_service_call {
    ($fn_name:ident, $req_ty:ty, $rep_ty:ty, $variant:ident)={
        pub async fn $fn_name(
            client_pool:&ClientPool,
            addrs:&[impl AsRef<str>],
            request:$req_ty,
        )->Result<$rep_ty,CommonError>{
            $crate::utils::retry_call(client_pool,addrs,&request).await
        }
    };
}
