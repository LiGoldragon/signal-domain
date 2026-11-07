//! The collections that were type aliases of Vec<DomainScope> are now newtypes;
//! their stored representation, archive bytes and datom text, is the Vec's own.

use signal_domain::{ArtDomain, Domain, DomainScope, DomainScopes, ScopeSet};

fn samples() -> Vec<Vec<DomainScope>> {
    let scope = |domain| DomainScope { domain };
    vec![
        vec![],
        vec![scope(Domain::All)],
        vec![
            scope(Domain::Art(ArtDomain::Music)),
            scope(Domain::All),
            scope(Domain::Art(ArtDomain::Dance)),
        ],
    ]
}

fn archive<T>(value: &T) -> Vec<u8>
where
    T: for<'a> rkyv::Serialize<
            rkyv::api::high::HighSerializer<
                rkyv::util::AlignedVec,
                rkyv::ser::allocator::ArenaHandle<'a>,
                rkyv::rancor::Error,
            >,
        >,
{
    rkyv::to_bytes::<rkyv::rancor::Error>(value)
        .expect("archive")
        .to_vec()
}

#[test]
fn the_newtypes_archive_to_the_bytes_of_the_vector_they_replaced() {
    for sample in samples() {
        let vector = archive(&sample);
        assert_eq!(archive(&DomainScopes(sample.clone())), vector);
        assert_eq!(archive(&ScopeSet(sample.clone())), vector);
        let restored = rkyv::from_bytes::<DomainScopes, rkyv::rancor::Error>(&vector)
            .expect("the vector's archive reads as the newtype");
        assert_eq!(restored.0, sample);
    }
}

#[cfg(feature = "datom")]
#[test]
fn the_newtypes_write_and_read_the_datom_text_of_the_vector_they_replaced() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    for sample in samples() {
        let vector_text = sample.clone().datomize(vec![]).protosize().textualize();
        assert_eq!(
            DomainScopes(sample.clone())
                .datomize(vec![])
                .protosize()
                .textualize(),
            vector_text
        );
        let mut pending = Potential::<DomainScopes>::from(vector_text);
        let restored = pending
            .actualize(&mut Budget {
                remaining: 1024,
                reader: ReaderBudget::try_from(1024).expect("bounded reader budget"),
                depth: 0,
                maximum_depth: 1024,
            })
            .expect("the vector's text reads as the newtype");
        assert_eq!(restored.0, sample);
    }
}
