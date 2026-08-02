#!/usr/bin/env bash

generate_fallback_changelog() {
    local range="$1" added="" changed="" fixed="" other=""
    local line type subject

    while IFS= read -r line; do
        [ -z "$line" ] && continue
        if [[ "$line" =~ ^([a-z]+)(\([^\)]*\))?!?:\ (.*)$ ]]; then
            type="${BASH_REMATCH[1]}"
            subject="${BASH_REMATCH[3]}"
        else
            type="other"
            subject="$line"
        fi
        case "$type" in
            feat) added+="- ${subject}"$'\n' ;;
            fix) fixed+="- ${subject}"$'\n' ;;
            perf|refactor) changed+="- ${subject}"$'\n' ;;
            chore|docs|test|build|ci|style) ;;
            *) other+="- ${subject}"$'\n' ;;
        esac
    done < <(git log --no-merges --pretty=tformat:'%s' "$range")

    local output=""
    [ -n "$added" ] && output+="### Added"$'\n'"$added"$'\n'
    [ -n "$changed" ] && output+="### Changed"$'\n'"$changed"$'\n'
    [ -n "$fixed" ] && output+="### Fixed"$'\n'"$fixed"$'\n'
    [ -n "$other" ] && output+="### Other"$'\n'"$other"$'\n'
    [ -z "$output" ] && output="### Changed"$'\n'"- Maintenance release."$'\n'
    printf '%s' "$output"
}

build_ai_prompt() {
    local range="$1" version="$2" commits
    commits="$(git log --no-merges --pretty=format:'- %s%n%b' "$range")"
    cat <<EOF
Write the changelog for Zokute $version, a desktop system monitor made of
always-on-desktop widgets. Turn the git log below into concise, user-facing
Markdown that follows Keep a Changelog.

Output only "### Added", "### Changed", "### Fixed", or "### Removed" sections.
Do not output a version header. Combine related commits, omit maintenance noise,
and use one present-tense bullet per change without hashes or trailing periods.
If nothing user-facing changed, output "### Changed" with one bullet.

$commits
EOF
}

call_chat_completions() {
    local base_url="$1" api_key="$2" model="$3" prompt="$4"
    local payload response
    command -v jq >/dev/null && command -v curl >/dev/null || return 1
    payload="$(jq -n --arg model "$model" --arg content "$prompt" \
        '{model: $model, temperature: 0.3, messages: [{role: "user", content: $content}]}')"
    response="$(curl -sS --fail "$base_url/chat/completions" \
        -H "Authorization: Bearer $api_key" -H "Content-Type: application/json" \
        -d "$payload" 2>/dev/null)" || return 1
    printf '%s' "$response" | jq -r '.choices[0].message.content // empty'
}

generate_with_opencode() {
    local prompt="$1"
    if [ -z "${OPENCODE_API_KEY:-}" ] || [ -z "${CHANGELOG_AI_MODEL:-}" ]; then
        warn "opencode needs OPENCODE_API_KEY and CHANGELOG_AI_MODEL; using fallback."
        return 1
    fi
    call_chat_completions "${OPENCODE_BASE_URL:-https://opencode.ai/zen/go/v1}" \
        "$OPENCODE_API_KEY" "$CHANGELOG_AI_MODEL" "$prompt"
}

generate_with_openai() {
    local prompt="$1"
    if [ -z "${OPENAI_API_KEY:-}" ]; then
        warn "openai needs OPENAI_API_KEY; using fallback."
        return 1
    fi
    call_chat_completions "${OPENAI_BASE_URL:-https://api.openai.com/v1}" \
        "$OPENAI_API_KEY" "${CHANGELOG_AI_MODEL:-gpt-4o-mini}" "$prompt"
}

generate_changelog() {
    local range="$1" version="$2" provider="${CHANGELOG_AI_PROVIDER:-}"
    local prompt result=""
    if [ -n "$provider" ]; then
        prompt="$(build_ai_prompt "$range" "$version")"
        case "$provider" in
            opencode) result="$(generate_with_opencode "$prompt")" || result="" ;;
            openai) result="$(generate_with_openai "$prompt")" || result="" ;;
            *) warn "Unknown AI provider '$provider'; using fallback." ;;
        esac
        result="$(printf '%s' "$result" | sed -e 's/[[:space:]]*$//')"
        if [ -n "$result" ] && printf '%s' "$result" | grep -q '^### '; then
            info "Changelog generated with $provider." >&2
            printf '%s\n' "$result"
            return
        fi
    fi
    generate_fallback_changelog "$range"
}
