_pac() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            pac)
                cmd="pac"
                ;;
            
            completions)
                cmd+="__completions"
                ;;
            generate)
                cmd+="__generate"
                ;;
            help)
                cmd+="__help"
                ;;
            install)
                cmd+="__install"
                ;;
            list)
                cmd+="__list"
                ;;
            move)
                cmd+="__move"
                ;;
            uninstall)
                cmd+="__uninstall"
                ;;
            update)
                cmd+="__update"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        pac)
            opts=" -h -V  --help --version   list install uninstall move update generate completions help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        
        pac__completions)
            opts=" -h -V  --help --version  <SHELL> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        pac__generate)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        pac__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        pac__install)
            opts=" -o -h -V -c -j  --opt --help --version --category --as --on --for --build --threads  <package>... "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --category)
                    COMPREPLY=("<CATEGORY>")
                    return 0
                    ;;
                    -c)
                    COMPREPLY=("<CATEGORY>")
                    return 0
                    ;;
                --as)
                    COMPREPLY=("<NAME>")
                    return 0
                    ;;
                --on)
                    COMPREPLY=("<LOAD_CMD>")
                    return 0
                    ;;
                --for)
                    COMPREPLY=("<TYPES>")
                    return 0
                    ;;
                --build)
                    COMPREPLY=("<BUILD_CMD>")
                    return 0
                    ;;
                --threads)
                    COMPREPLY=("<THREADS>")
                    return 0
                    ;;
                    -j)
                    COMPREPLY=("<THREADS>")
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        pac__list)
            opts=" -s -o -d -h -V -c  --start --opt --detached --help --version --category  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --category)
                    COMPREPLY=("<CATEGORY>")
                    return 0
                    ;;
                    -c)
                    COMPREPLY=("<CATEGORY>")
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        pac__move)
            opts=" -o -h -V  --opt --help --version  <package> <category> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        pac__uninstall)
            opts=" -h -V  --help --version  <package>... "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        pac__update)
            opts=" -s -j -h -V  --skip --threads --help --version  <package>... "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
    esac
}

complete -F _pac -o bashdefault -o default pac
