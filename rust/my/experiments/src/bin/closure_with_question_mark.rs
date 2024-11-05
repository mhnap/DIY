fn main() {
    struct SVM {
        endpoints: Option<Endpoints>,
    }

    struct Endpoints {
        nfs: Option<NFS>,
    }

    struct NFS {
        ip_addresses: Option<Vec<String>>,
    }

    #[derive(Debug)]
    struct MyError {
        #[expect(dead_code)]
        message: String,
    }

    impl MyError {
        fn new(msg: &str) -> Self {
            MyError { message: msg.to_string() }
        }
    }

    fn get_first_ip_with_methods(svm: SVM) -> Result<String, MyError> {
        svm.endpoints
            .and_then(|endpoints| {
                endpoints.nfs.and_then(|nfs| {
                    nfs.ip_addresses.and_then(|mut ip_addresses| ip_addresses.pop())
                })
            })
            .ok_or(MyError::new("Failed to pop an IP address"))
    }

    // Can hack by creating closure and immediately calling it.
    fn get_first_ip_with_closure(svm: SVM) -> Result<String, MyError> {
        (|| svm.endpoints?.nfs?.ip_addresses?.pop())()
            .ok_or(MyError::new("Failed to pop an IP address"))
    }

    let _ = dbg!(get_first_ip_with_methods(SVM {
        endpoints: Some(Endpoints {
            nfs: Some(NFS { ip_addresses: Some(vec!["0.0.0.0".to_string()]) }),
        }),
    }));

    let _ = dbg!(get_first_ip_with_closure(SVM {
        endpoints: Some(Endpoints {
            nfs: Some(NFS { ip_addresses: Some(vec!["0.0.0.0".to_string()]) }),
        }),
    }));

    let _ = dbg!(get_first_ip_with_closure(SVM {
        endpoints: Some(Endpoints { nfs: Some(NFS { ip_addresses: Some(vec![]) }) })
    }));
}
