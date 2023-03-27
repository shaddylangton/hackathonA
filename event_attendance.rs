use ink_lang::contract;

contract! {
    #![env = ink_env::DefaultSrmlTypes]

    struct EventAttendance {
        attendees: ink_storage::collections::Vec<AccountId>,
        tokens: ink_storage::collections::HashMap<u32, AccountId>,
    }

    impl EventAttendance {
        #[ink(constructor)]
        fn new(&mut self) {
            self.attendees.push(Self::env().caller());
        }

        #[ink(message)]
        fn register_attendee(&mut self) {
            let caller = Self::env().caller();
            if !self.attendees.contains(&caller) {
                self.attendees.push(caller);
            }
        }

        #[ink(message)]
        fn issue_tokens(&mut self, count: u32) {
            let caller = Self::env().caller();
            let mut token_id = 0;
            for attendee in self.attendees.iter() {
                if *attendee == caller {
                    continue;
                }
                for _ in 0..count {
                    self.tokens.insert(token_id, *attendee);
                    token_id += 1;
                }
            }
        }

        #[ink(message)]
        fn verify_attendance(&self, attendee: AccountId, token_id: u32) -> bool {
            match self.tokens.get(&token_id) {
                Some(address) => *address == attendee && self.attendees.contains(&attendee),
                None => false,
            }
        }
    }
}
