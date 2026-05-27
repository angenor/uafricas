<template>
  <aside
    class="flex flex-col w-80 bg-white border-l border-gray-200 text-gray-900 shadow-lg"
    aria-label="Panneau de modération de session"
  >
    <header class="flex items-center justify-between px-4 py-3 border-b border-gray-200 bg-gray-50">
      <div class="flex items-center gap-2">
        <font-awesome-icon :icon="['fas', 'shield-halved']" class="w-4 h-4 text-custom-chocolat" />
        <h2 class="font-semibold text-sm">Modération de session</h2>
      </div>
      <button
        type="button"
        class="inline-flex items-center justify-center w-7 h-7 rounded text-gray-500 hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-gray-300 transition"
        aria-label="Fermer le panneau"
        @click="$emit('fermer')"
      >
        <font-awesome-icon :icon="['fas', 'xmark']" class="w-3 h-3" />
      </button>
    </header>

    <div class="px-4 py-2 text-xs text-gray-600 bg-amber-50 border-b border-amber-100">
      <p>
        Vous êtes
        <span class="font-semibold">{{ libelleNiveau(monNiveauModerateurSession) }}</span>
        pour cette session.
      </p>
    </div>

    <section class="flex-1 overflow-y-auto">
      <h3 class="px-4 py-2 text-xs uppercase tracking-wide text-gray-500 font-semibold">
        Permissions tableau blanc
      </h3>

      <div v-if="erreurAffichee" class="mx-4 mb-2 px-3 py-2 text-xs rounded bg-red-50 border border-red-200 text-red-700">
        {{ erreurAffichee }}
      </div>

      <ul class="divide-y divide-gray-100">
        <li
          v-for="m in moderateursOffice"
          :key="`m-${m.utilisateur_id}`"
          class="flex items-center gap-3 px-4 py-2"
        >
          <div class="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-xs font-semibold text-gray-700 overflow-hidden">
            <img
              v-if="m.avatar_url"
              :src="m.avatar_url"
              :alt="m.nom_complet"
              class="w-full h-full object-cover"
            >
            <span v-else>{{ initiales(m.nom_complet) }}</span>
          </div>
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium truncate">{{ m.nom_complet || m.utilisateur_id }}</p>
            <p class="text-[11px] text-custom-chocolat font-semibold uppercase tracking-wide">
              {{ libelleNiveau(m.niveau) }}
            </p>
          </div>
          <span
            class="text-[11px] px-2 py-0.5 rounded-full bg-gray-100 text-gray-600 font-medium"
            title="Permission incluse par le rôle de modérateur"
          >
            Auto
          </span>
        </li>

        <li
          v-for="p in participantsToggleables"
          :key="`p-${p.identity}`"
          class="flex items-center gap-3 px-4 py-2"
        >
          <div class="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-xs font-semibold text-gray-700">
            {{ initiales(p.name) }}
          </div>
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium truncate">{{ p.name || p.identity }}</p>
            <p v-if="p.isLocal" class="text-[11px] text-gray-400">Vous</p>
          </div>
          <label class="inline-flex items-center cursor-pointer">
            <input
              type="checkbox"
              class="sr-only peer"
              :checked="estAutorise(p.identity)"
              :disabled="enCours.has(p.identity)"
              @change="basculer(p)"
            >
            <span
              class="w-9 h-5 rounded-full bg-gray-300 peer-checked:bg-custom-green relative transition-colors"
              :class="{ 'opacity-50': enCours.has(p.identity) }"
            >
              <span
                class="absolute top-0.5 left-0.5 w-4 h-4 bg-white rounded-full transition-transform"
                :class="{ 'translate-x-4': estAutorise(p.identity) }"
              />
            </span>
          </label>
        </li>

        <li v-if="!moderateursOffice.length && !participantsToggleables.length" class="px-4 py-6 text-center text-xs text-gray-400">
          Aucun participant connecté.
        </li>
      </ul>

      <!-- Section spotlight (FR-001b + FR-027) :
           visible uniquement pour admin_plateforme | admin_salle ET session publique. -->
      <template v-if="peutSpotlight && estSessionPublique">
        <h3 class="px-4 py-2 mt-4 text-xs uppercase tracking-wide text-gray-500 font-semibold border-t border-gray-100">
          Mise en évidence
        </h3>

        <div
          v-if="erreurSpotlight"
          class="mx-4 mb-2 px-3 py-2 text-xs rounded bg-red-50 border border-red-200 text-red-700"
        >
          {{ erreurSpotlight }}
        </div>

        <div
          v-if="spotlightActif"
          class="mx-4 mb-2 px-3 py-2 text-xs rounded bg-amber-50 border border-amber-200 text-amber-800 flex items-center justify-between gap-2"
        >
          <span class="flex items-center gap-2 min-w-0">
            <font-awesome-icon :icon="['fas', 'star']" class="w-3 h-3 text-custom-chocolat shrink-0" />
            <span class="font-medium truncate">{{ spotlightActif.nom_complet || spotlightActif.utilisateur_id }}</span>
          </span>
          <button
            type="button"
            class="text-xs px-2 py-1 rounded bg-white border border-gray-300 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-gray-300 transition shrink-0"
            :disabled="spotlightEnCours"
            @click="desactiverSpotlight"
          >
            Désactiver
          </button>
        </div>

        <ul class="divide-y divide-gray-100">
          <li
            v-for="p in participants"
            :key="`s-${p.identity}`"
            class="flex items-center gap-3 px-4 py-2"
          >
            <div class="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-xs font-semibold text-gray-700">
              {{ initiales(p.name) }}
            </div>
            <div class="flex-1 min-w-0">
              <p class="text-sm font-medium truncate">{{ p.name || p.identity }}</p>
              <p v-if="p.isLocal" class="text-[11px] text-gray-400">Vous</p>
            </div>
            <button
              v-if="!estEnVedette(p.identity)"
              type="button"
              class="text-xs px-2 py-1 rounded bg-custom-chocolat text-white hover:bg-amber-800 focus:outline-none focus:ring-2 focus:ring-amber-300 transition shrink-0"
              :disabled="spotlightEnCours"
              @click="activerSpotlight(p)"
            >
              <font-awesome-icon :icon="['fas', 'star']" class="w-3 h-3 mr-1" />
              Mettre en évidence
            </button>
            <span
              v-else
              class="text-[11px] px-2 py-0.5 rounded-full bg-amber-100 text-amber-800 font-medium"
            >
              En vedette
            </span>
          </li>
        </ul>
      </template>
    </section>
  </aside>
</template>

<script setup lang="ts">
import type { NiveauModerateur } from '~/composables/useAfrolang'
import type { RoomParticipant } from '~/components/afrolang/AfrolangRoom.vue'

const props = defineProps<{
  sessionId: string
  participants: RoomParticipant[]
  /** Session publique (FR-027 — spotlight indisponible en privé). */
  estSessionPublique: boolean
}>()

defineEmits<{
  fermer: []
}>()

const {
  monNiveauModerateurSession,
  permissionsTableauBlanc,
  moderateursOffice,
  spotlightActif,
  accorderPermissionTableauBlanc,
  retirerPermissionTableauBlanc,
  mettreEnEvidence,
  retirerMiseEnEvidence,
} = useAfrolang()

const spotlightEnCours = ref(false)
const erreurSpotlight = ref('')

/** FR-001b : seuls admin_plateforme et admin_salle peuvent activer le spotlight. */
const peutSpotlight = computed(() => {
  const n = monNiveauModerateurSession.value
  return n === 'admin_plateforme' || n === 'admin_salle'
})

function estEnVedette(identity: string): boolean {
  return spotlightActif.value?.utilisateur_id === identity
}

async function activerSpotlight(p: RoomParticipant): Promise<void> {
  erreurSpotlight.value = ''
  spotlightEnCours.value = true
  try {
    const res = await mettreEnEvidence(props.sessionId, p.identity)
    if (typeof res === 'object' && 'erreur' in res) {
      erreurSpotlight.value = libelleErreurSpotlight(res.erreur)
    }
  }
  finally {
    spotlightEnCours.value = false
  }
}

async function desactiverSpotlight(): Promise<void> {
  erreurSpotlight.value = ''
  spotlightEnCours.value = true
  try {
    const res = await retirerMiseEnEvidence(props.sessionId)
    if (typeof res === 'object' && 'erreur' in res) {
      erreurSpotlight.value = libelleErreurSpotlight(res.erreur)
    }
  }
  finally {
    spotlightEnCours.value = false
  }
}

function libelleErreurSpotlight(code: string): string {
  switch (code) {
    case 'interdit': return 'Capacité réservée aux admins.'
    case 'session_privee': return 'Spotlight indisponible en session privée.'
    case 'absent': return 'Le participant n\'est plus dans la session.'
    default: return 'Échec de l\'opération.'
  }
}

const enCours = ref(new Set<string>())
const erreurAffichee = ref('')

function initiales(nom: string | null | undefined): string {
  if (!nom) return '?'
  return nom
    .split(/\s+/)
    .map(s => s[0])
    .filter(Boolean)
    .slice(0, 2)
    .join('')
    .toUpperCase()
}

function libelleNiveau(n: NiveauModerateur | null | undefined): string {
  switch (n) {
    case 'admin_plateforme': return 'Admin plateforme'
    case 'admin_salle': return 'Admin salle'
    case 'moderateur_attitre': return 'Modérateur attitré'
    case 'createur_salle_privee': return 'Créateur de salle'
    default: return 'Non modérateur'
  }
}

function estModerateurOffice(identity: string): boolean {
  return moderateursOffice.value.some(m => m.utilisateur_id === identity)
}

function estAutorise(identity: string): boolean {
  return permissionsTableauBlanc.value.some(p => p.utilisateur_id === identity)
}

// Participants à exposer (toggleables) : exclure les modérateurs d'office.
const participantsToggleables = computed(() =>
  props.participants.filter(p => !estModerateurOffice(p.identity)),
)

async function basculer(p: RoomParticipant): Promise<void> {
  erreurAffichee.value = ''
  enCours.value = new Set([...enCours.value, p.identity])
  try {
    if (estAutorise(p.identity)) {
      const res = await retirerPermissionTableauBlanc(props.sessionId, p.identity)
      if (typeof res === 'object' && 'erreur' in res) {
        erreurAffichee.value = res.erreur === 'conflit'
          ? 'Cette permission ne peut pas être retirée à un modérateur.'
          : 'Erreur lors du retrait.'
      }
    }
    else {
      const res = await accorderPermissionTableauBlanc(props.sessionId, p.identity)
      if (typeof res === 'object' && 'erreur' in res) {
        erreurAffichee.value = res.erreur === 'conflit'
          ? 'L\'utilisateur est déjà modérateur de session.'
          : 'Erreur lors de l\'attribution.'
      }
    }
  }
  finally {
    const next = new Set(enCours.value)
    next.delete(p.identity)
    enCours.value = next
  }
}
</script>
