; Queries for nvim-treesitter/nvim-treesitter-textobjects
;--------------------------------------------------------

; Classes (modules)
;------------------

(module_binding definition: ((_) @class.inside)) @class.around

; Blocks
;-------

(block (_) @block.inside) @block.around

; Functions
;----------

(function body: (_) @function.inside) @function.around

; Calls
;------

(call_expression arguments: ((_) @call.inside)) @call.around

; Comments
;---------

(comment) @comment.around

; Parameters
;-----------

(function parameter: (_) @parameter.inside @parameter.around)

(formal_parameters
  "," @_formal_parameters_start
  . (_) @parameter.inside
  (#make-range! "parameter.around" @_formal_parameters_start @parameter.inside))
(formal_parameters
  . (_) @parameter.inside
  . ","? @_formal_parameters_end
  (#make-range! "parameter.around" @parameter.inside @_formal_parameters_end))

(arguments
  "," @_arguments_start
  . (_) @parameter.inside
  (#make-range! "parameter.around" @_arguments_start @parameter.inside))
(arguments
  . (_) @parameter.inside
  . ","? @_arguments_end
  (#make-range! "parameter.around" @parameter.inside @_arguments_end))

(function_type_parameters
  "," @_function_type_parameters_start
  . (_) @parameter.inside
  (#make-range! "parameter.around" @_function_type_parameters_start @parameter.inside))
(function_type_parameters
  . (_) @parameter.inside
  . ","? @_function_type_parameters_end
  (#make-range! "parameter.around" @parameter.inside @_function_type_parameters_end))

(functor_parameters
  "," @_functor_parameters_start
  . (_) @parameter.inside
  (#make-range! "parameter.around" @_functor_parameters_start @parameter.inside))
(functor_parameters
  . (_) @parameter.inside
  . ","? @_functor_parameters_end
  (#make-range! "parameter.around" @parameter.inside @_functor_parameters_end))

(type_parameters
  "," @_type_parameters_start
  . (_) @parameter.inside
  (#make-range! "parameter.around" @_type_parameters_start @parameter.inside))
(type_parameters
  . (_) @parameter.inside
  . ","? @_type_parameters_end
  (#make-range! "parameter.around" @parameter.inside @_type_parameters_end))

(type_arguments
  "," @_type_arguments_start
  . (_) @parameter.inside
  (#make-range! "parameter.around" @_type_arguments_start @parameter.inside))
(type_arguments
  . (_) @parameter.inside
  . ","? @_type_arguments_end
  (#make-range! "parameter.around" @parameter.inside @_type_arguments_end))

(decorator_arguments
  "," @_decorator_arguments_start
  . (_) @parameter.inside
  (#make-range! "parameter.around" @_decorator_arguments_start @parameter.inside))
(decorator_arguments
  . (_) @parameter.inside
  . ","? @_arguments_end
  (#make-range! "parameter.around" @parameter.inside @_arguments_end))

(variant_parameters
  "," @_variant_parameters_start
  . (_) @parameter.inside
  (#make-range! "parameter.around" @_variant_parameters_start @parameter.inside))
(variant_parameters
  . (_) @parameter.inside
  . ","? @_variant_parameters_end
  (#make-range! "parameter.around" @parameter.inside @_variant_parameters_end))

(polyvar_parameters
  "," @_polyvar_parameters_start
  . (_) @parameter.inside
  (#make-range! "parameter.around" @_polyvar_parameters_start @parameter.inside))
(polyvar_parameters
  . (_) @parameter.inside
  . ","? @_polyvar_parameters_end
  (#make-range! "parameter.around" @parameter.inside @_polyvar_parameters_end))
