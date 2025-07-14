#[macro_export]
macro_rules! test_core_entity {
    (
        $entity:ident,
        $child_singular:ident,
        $child_plural:ident,
        $child_ty:ty,
        $child_new:expr
    ) => {
        paste::paste! {
            #[test]
            fn [<creates_ $entity:lower _with_name>]() {
                let e = $entity::new("foo");
                assert_eq!(e.name(), "foo");
                assert!(e.$child_plural().is_empty());
            }

            #[test]
            fn [<adds_and_removes_ $child_singular>]() {
                let mut e = $entity::new("foo");
                let c: $child_ty = $child_new;
                let id = c.id();
                e.[<add_ $child_singular>](c);
                assert_eq!(e.$child_plural().len(), 1);
                let removed = e.[<remove_ $child_singular _by_id>](id);
                assert!(removed.is_some());
                assert!(e.$child_plural().is_empty());
            }

            #[test]
            fn [<adds_multiple_ $child_plural>]() {
                let mut e = $entity::new("foo");
                let c1: $child_ty = $child_new;
                let c2: $child_ty = $child_new;
                e.[<add_ $child_plural>](vec![c1.clone(), c2.clone()]);
                assert_eq!(e.$child_plural().len(), 2);
            }

            #[test]
            fn [<clears_ $child_plural>]() {
                let mut e = $entity::new("foo");
                let c: $child_ty = $child_new;
                e.[<add_ $child_singular>](c);
                e.clear();
                assert!(e.$child_plural().is_empty());
            }

            #[test]
            fn [<renames_ $entity:lower>]() {
                let mut e = $entity::new("foo");
                e.rename("bar");
                assert_eq!(e.name(), "bar");
            }

            #[test]
            fn [<sets_and_clears_ $entity:lower _description>]() {
                let mut e = $entity::new("foo");
                e.set_description("desc");
                assert_eq!(e.description(), Some("desc"));
                e.clear_description();
                assert_eq!(e.description(), None);
            }

            #[test]
            fn [<gets_ $child_singular _by_id>]() {
                let mut e = $entity::new("foo");
                let c: $child_ty = $child_new;
                let id = c.id();
                e.[<add_ $child_singular>](c.clone());
                let found = e.[<get_ $child_singular _by_id>](id);
                assert!(found.is_some());
                assert_eq!(found.unwrap().id(), id);
                assert_eq!(found.unwrap(), &c);
                let not_found = e.[<get_ $child_singular _by_id>](uuid::Uuid::new_v4());
                assert!(not_found.is_none());
            }

            #[test]
            fn [<gets_ $child_singular _by_id_mut>]() {
                let mut e = $entity::new("foo");
                let c: $child_ty = $child_new;
                let id = c.id();
                e.[<add_ $child_singular>](c.clone());
                {
                    let found_mut = e.[<get_ $child_singular _by_id_mut>](id);
                    assert!(found_mut.is_some());
                    // Optionally mutate here if $child_ty supports it
                }
                let not_found_mut = e.[<get_ $child_singular _by_id_mut>](uuid::Uuid::new_v4());
                assert!(not_found_mut.is_none());
            }

            #[test]
            fn [<gets_ $child_singular _index_by_id>]() {
                let mut e = $entity::new("foo");
                let c: $child_ty = $child_new;
                let id = c.id();
                e.[<add_ $child_singular>](c.clone());
                let idx = e.[<get_ $child_singular _index_by_id>](id);
                assert_eq!(idx, Some(0));
                let not_found = e.[<get_ $child_singular _index_by_id>](uuid::Uuid::new_v4());
                assert!(not_found.is_none());
            }
        }
    };
}
