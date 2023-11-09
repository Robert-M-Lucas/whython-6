#[macro_export]
macro_rules! default_type_wrapper_struct_and_impl {
    ($wrapper_name: ident, $type_name: ident, $type_symbol: expr) => {
        pub struct $wrapper_name {}

        impl $crate::processing::types::UninstantiatedType for $wrapper_name {
            fn instantiate(&self) -> Box<dyn $crate::processing::types::Type> {
                bx!($type_name::new())
            }

            fn get_type_symbol(&self) -> $crate::processing::symbols::TypeSymbol {
                $type_symbol
            }
        }
    };
}

#[macro_export]
macro_rules! default_type_struct {
    ($type_name: ident) => {
        #[allow(dead_code)]
        pub struct $type_name {
            operators: Vec<Box<dyn $crate::processing::types::Operation<$type_name>>>,
            operators_prefix: Vec<Box<dyn $crate::processing::types::PrefixOperation<$type_name>>>,
            address: Option<$crate::address::Address>,
        }
    };
}

#[macro_export]
macro_rules! default_type_initialiser {
    ($type_name: ident,  ($( $operator: ident ),*), ($( $operator_prefix: ident ),*)) => {
        impl $type_name {
            pub fn new() -> Self {
                Self {
                    operators: vec![
                        $($crate::bx!($operator{}),
                    )*
                    ],
                    operators_prefix: vec![
                        $($crate::bx!($operator_prefix{}),
                    )*
                    ],
                    address: None
                }
            }
        }

        impl Default for $type_name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

#[macro_export]
macro_rules! default_get_type_symbol_impl {
    ($type_name: ident, $type_symbol: expr) => {
        fn get_type_symbol(&self) -> $crate::processing::symbols::TypeSymbol {
            $type_symbol
        }
    };
}

#[macro_export]
macro_rules! default_type_operate_impl {
    ($type_name: ident) => {
        fn get_prefix_operation_result_type(&self, operator: &Operator) -> Vec<TypeSymbol> {
            let mut results = Vec::new();

            for op in self.operators_prefix.iter() {
                if op.get_symbol() == *operator {
                    if let Some(result) = op.get_result_type() {
                        results.push(result);
                    }
                }
            }

            results
        }

        fn get_operation_result_type(
            &self,
            operator: &Operator,
            rhs: &TypeSymbol,
        ) -> Vec<TypeSymbol> {
            let mut results = Vec::new();

            for op in self.operators.iter() {
                if op.get_symbol() == *operator {
                    if let Some(result) = op.get_result_type(rhs) {
                        results.push(result);
                    }
                }
            }

            results
        }

        fn operate_prefix(
            &self,
            operator: &$crate::processing::symbols::Operator,
            destination: &dyn $crate::processing::types::Type,
            program_memory: &mut $crate::memory::MemoryManager,
            stack_sizes: &mut $crate::processing::blocks::StackSizes,
        ) -> Result<(), String> {
            for op in self.operators_prefix.iter() {
                if op.get_symbol() == *operator && op.get_result_type().is_some() {
                    return op.operate_prefix(self, destination, program_memory, stack_sizes);
                }
            }

            Err(format!(
                "Operator {} not supported on {}",
                operator,
                self.get_type_symbol()
            ))
        }

        fn operate(
            &self,
            operator: &$crate::processing::symbols::Operator,
            rhs: &dyn $crate::processing::types::Type,
            destination: &dyn $crate::processing::types::Type,
            program_memory: &mut $crate::memory::MemoryManager,
            stack_sizes: &mut $crate::processing::blocks::StackSizes,
        ) -> Result<(), String> {
            for op in self.operators.iter() {
                if op.get_symbol() == *operator
                    && op.get_result_type(&rhs.get_type_symbol()).is_some()
                {
                    return op.operate(self, rhs, destination, program_memory, stack_sizes);
                }
            }

            Err(format!(
                "Operator {} not supported between {} and {}",
                operator,
                self.get_type_symbol(),
                rhs.get_type_symbol()
            ))
        }
    };
}
