(use-modules (gnu)
             (gnu packages protobuf)
             (gnu packages crates-io)
             (gnu packages crates-tls)
             (gnu packages crates-web)
             (gnu packages rust)
             (guix build-system cargo)
             (guix download)
             (guix gexp)
             (guix packages)
             (guix utils)
             (ice-9 match)
             (srfi srfi-1))

(define-public rust-mio-aio-0.8
  (package
    (inherit rust-mio-aio-0.7)
    (name "rust-mio-aio")
    (version "0.8.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mio-aio" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1ynga39vd3r7i3bjqsiv8b6b9z8ympby88l7vkk5cvhp6kn3livj"))))))

(define-public rust-tokio-1.36
  (package
    (inherit rust-tokio-1)
    (name "rust-tokio")
    (version "1.36.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tokio" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0c89p36zbd4abr1z3l5mipp43x7z4c9b4vp4s6r8y0gs2mjmya31"))))
    (arguments
     (substitute-keyword-arguments (package-arguments rust-tokio-1)
       ((#:cargo-inputs original-inputs)
        (assoc-set! original-inputs
                    "rust-mio-aio"
                    `(,rust-mio-aio-0.8)))))))

(define-public rust-h2-0.3.24
  (package
    (inherit rust-h2-0.3)
    (name "rust-h2")
    (version "0.3.24")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "h2" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1jf9488b66nayxzp3iw3b2rb64y49hdbbywnv9wfwrsv14i48b5v"))))))

(define-public rust-tonic-0.11
  (package
    (inherit rust-tonic-0.10)
    (name "rust-tonic")
    (version "0.11.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tonic" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "04qsr527i256i3dk9dp1g2jr42q7yl91y5h06rvd9ycy9rxfpi3n"))))
    (arguments
     (substitute-keyword-arguments (package-arguments rust-tonic-0.10)
       ((#:cargo-inputs original-inputs)
        (fold (lambda (replacement inputs)
                (match replacement
                  ((name gexp)
                   (assoc-set! inputs name (list gexp)))))
              original-inputs
              `(("rust-h2" ,rust-h2-0.3.24)
                ("rustls-native-certs" ,rust-rustls-native-certs-0.7)
                ("rust-tokio-rustls" ,rust-tokio-rustls-0.25)
                ("rust-webpki-roots" ,rust-webpki-roots-0.26)
                ("rust-zstd" ,rust-zstd-0.12))))))))

(define-public rust-tonic-build-0.11
  (package
    (inherit rust-tonic-build-0.10)
    (name "rust-tonic-build")
    (version "0.11.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tonic-build" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1hm99ckaw0pzq8h22bdjy6gpbg06kpvs0f73nj60f456f3fzckmy"))))))

(define-public orcanet-market
  (package
   (name "orcanet-market")
   (version "0.1")
   (source (local-file (dirname (current-filename)) #:recursive? #t))
   (build-system cargo-build-system)
   (arguments
    (list
     #:cargo-inputs
     `(("rust-prost" ,rust-prost-0.12)
       ("rust-tokio" ,rust-tokio-1.36)
       ("rust-tonic" ,rust-tonic-0.11)
       ("rust-tonic-build" ,rust-tonic-build-0.11))))
   (native-inputs
    (list protobuf))
   (synopsis "TODO")
   (description "TODO")
   (license #f)
   (home-page "TODO")))

orcanet-market
