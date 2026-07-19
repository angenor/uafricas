<script setup lang="ts">
/**
 * Détenteurs d'un support média — chaîne ou station (US5, T099).
 *
 * Le retrait est un soft delete côté serveur : les anciens détenteurs restent
 * listés à part. Savoir qui a tenu un support, et jusqu'à quand, fait partie de
 * sa fiche d'identité ; effacer l'historique en ferait un support sans passé.
 */
import type { DetenteurAPI, RoleDetenteur, TypeSupportMedia } from '~/composables/useMediaDetention'

const props = withDefaults(defineProps<{
  typeSupport: TypeSupportMedia
  supportId: string
  monRole?: RoleDetenteur | null
}>(), { monRole: null })

const emit = defineEmits<{ maj: [] }>()

const { chargement, erreur, listerDetenteurs, inviter, retirer } = useMediaDetention()
const userStore = useUserStore()

const detenteurs = ref<DetenteurAPI[]>([])

const charger = async () => {
  detenteurs.value = await listerDetenteurs(props.typeSupport, props.supportId)
}

onMounted(charger)

const actifs = computed(() => detenteurs.value.filter(d => d.actif))
const anciens = computed(() => detenteurs.value.filter(d => !d.actif))

/** Seul le propriétaire invite et révoque — la garde réelle est côté serveur. */
const peutGerer = computed(() => roleAuMoins(props.monRole, 'proprietaire'))

const nomComplet = (d: DetenteurAPI) =>
  [d.utilisateur_prenom, d.utilisateur_nom].filter(Boolean).join(' ') || 'Utilisateur'

/**
 * On ne propose pas de retirer sa propre ligne : un support sans propriétaire
 * n'aurait plus personne pour en désigner un nouveau.
 */
const estMoi = (d: DetenteurAPI) => d.utilisateur_id === userStore.user?.id

// ---------------------------------------------------------------------------
// Invitation
// ---------------------------------------------------------------------------

const email = ref('')
const roleInvite = ref<Exclude<RoleDetenteur, 'proprietaire'>>('co_detenteur')
const envoiEnCours = ref(false)
/** Message de confirmation, distinct de `erreur` porté par le composable. */
const confirmation = ref<string | null>(null)

const envoyerInvitation = async () => {
  if (!email.value.trim()) return
  envoiEnCours.value = true
  confirmation.value = null
  const resultat = await inviter(props.typeSupport, props.supportId, email.value.trim(), roleInvite.value)
  envoiEnCours.value = false
  if (!resultat) return

  // L'invitation vise une adresse, pas forcément un compte : si personne ne
  // répond à ce courriel sur la plateforme, elle attend son inscription.
  confirmation.value = resultat.destinataire_reconnu
    ? `Invitation envoyée à ${email.value.trim()}.`
    : `Invitation enregistrée pour ${email.value.trim()} : elle sera proposée dès l’inscription de cette personne.`

  email.value = ''
  await charger()
  emit('maj')
}

const retirerDetenteur = async (d: DetenteurAPI) => {
  if (!confirm(`Retirer ${nomComplet(d)} des détenteurs de ce support ?`)) return
  confirmation.value = null
  if (!await retirer(props.typeSupport, props.supportId, d.utilisateur_id)) return
  await charger()
  emit('maj')
}

const dateCourte = (valeur: string | null) => {
  if (!valeur) return ''
  return new Date(valeur).toLocaleDateString('fr-FR', { day: '2-digit', month: 'long', year: 'numeric' })
}
</script>

<template>
  <section class="text-white">
    <header class="mb-5">
      <h2 class="text-xl sm:text-2xl font-bold">Détenteurs du support</h2>
      <p class="text-sm text-gray-400">
        Les personnes autorisées à administrer cette chaîne ou cette station.
      </p>
    </header>

    <p
      v-if="erreur"
      class="mb-5 rounded-lg border border-red-500 bg-red-500/10 text-red-300 px-4 py-3 text-sm"
    >
      <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="mr-2" />
      {{ erreur }}
    </p>

    <p
      v-if="confirmation"
      class="mb-5 rounded-lg border border-yellow-400/50 bg-yellow-400/10 text-yellow-200 px-4 py-3 text-sm"
    >
      <font-awesome-icon :icon="['fas', 'circle-info']" class="mr-2" />
      {{ confirmation }}
    </p>

    <div v-if="chargement && !detenteurs.length" class="flex justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-yellow-400" />
    </div>

    <template v-else>
      <!-- Détenteurs en exercice -->
      <ul v-if="actifs.length" class="space-y-3">
        <li
          v-for="d in actifs"
          :key="d.id"
          class="flex items-center gap-4 rounded-xl border border-white/10 bg-neutral-900 px-4 py-3"
        >
          <img
            v-if="d.utilisateur_photo"
            :src="d.utilisateur_photo"
            :alt="nomComplet(d)"
            loading="lazy"
            class="h-11 w-11 rounded-full object-cover shrink-0"
          >
          <span
            v-else
            class="h-11 w-11 rounded-full shrink-0 bg-neutral-800 flex items-center justify-center"
          >
            <font-awesome-icon :icon="['fas', 'user']" class="text-neutral-600" />
          </span>

          <div class="min-w-0 flex-1">
            <p class="text-white font-semibold truncate">
              {{ nomComplet(d) }}
              <span v-if="estMoi(d)" class="text-xs font-normal text-gray-500">(vous)</span>
            </p>
            <p v-if="d.utilisateur_email" class="text-sm text-gray-400 truncate">
              {{ d.utilisateur_email }}
            </p>
          </div>

          <span
            class="shrink-0 rounded-full border border-yellow-400/50 bg-yellow-400/10 text-yellow-400 text-xs px-3 py-1 cursor-help"
            :title="DESCRIPTIONS_ROLE_DETENTEUR[d.role]"
          >
            {{ LIBELLES_ROLE_DETENTEUR[d.role] }}
          </span>

          <button
            v-if="peutGerer && !estMoi(d)"
            type="button"
            class="shrink-0 text-gray-500 hover:text-red-400 transition-colors"
            :title="`Retirer ${nomComplet(d)}`"
            @click="retirerDetenteur(d)"
          >
            <font-awesome-icon :icon="['fas', 'user-minus']" />
          </button>
        </li>
      </ul>

      <p
        v-else
        class="rounded-xl border border-white/10 bg-neutral-900 px-6 py-10 text-center text-gray-400"
      >
        Aucun détenteur actif pour ce support.
      </p>

      <!-- Invitation d'un co-détenteur ou d'un programmateur -->
      <form
        v-if="peutGerer"
        class="mt-6 rounded-xl border border-white/10 bg-neutral-900 p-5"
        @submit.prevent="envoyerInvitation"
      >
        <h3 class="text-sm font-bold uppercase tracking-wide text-gray-300 mb-4">
          Inviter une personne
        </h3>

        <div class="flex flex-col sm:flex-row gap-3">
          <input
            v-model="email"
            type="email"
            required
            placeholder="adresse@exemple.org"
            class="flex-1 rounded-lg bg-neutral-800 border border-white/10 text-white px-3 py-2 text-sm placeholder:text-gray-600 focus:border-yellow-400 focus:outline-none"
          >
          <!-- Le rôle `proprietaire` est absent : un support n'en a qu'un, et
               c'est l'auteur validé de la proposition (US4). -->
          <select
            v-model="roleInvite"
            class="rounded-lg bg-neutral-800 border border-white/10 text-white px-3 py-2 text-sm focus:border-yellow-400 focus:outline-none"
          >
            <option value="co_detenteur">{{ LIBELLES_ROLE_DETENTEUR.co_detenteur }}</option>
            <option value="programmateur">{{ LIBELLES_ROLE_DETENTEUR.programmateur }}</option>
          </select>
          <button
            type="submit"
            :disabled="envoiEnCours || !email.trim()"
            class="inline-flex items-center justify-center gap-2 rounded-full bg-yellow-400 text-neutral-900 font-semibold px-5 py-2 text-sm hover:bg-yellow-300 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span
              v-if="envoiEnCours"
              class="animate-spin rounded-full h-4 w-4 border-b-2 border-neutral-900"
            />
            <font-awesome-icon v-else :icon="['fas', 'paper-plane']" />
            Inviter
          </button>
        </div>

        <p class="mt-3 text-xs text-gray-500">
          {{ DESCRIPTIONS_ROLE_DETENTEUR[roleInvite] }}
        </p>
      </form>

      <!-- Historique : jamais effacé, seulement mis en retrait -->
      <div v-if="anciens.length" class="mt-8">
        <h3 class="text-sm font-bold uppercase tracking-wide text-gray-500 mb-3">
          Anciens détenteurs
        </h3>
        <ul class="space-y-2">
          <li
            v-for="d in anciens"
            :key="d.id"
            class="flex items-center gap-4 rounded-lg border border-white/5 bg-neutral-900/50 px-4 py-2.5 opacity-60"
          >
            <img
              v-if="d.utilisateur_photo"
              :src="d.utilisateur_photo"
              :alt="nomComplet(d)"
              loading="lazy"
              class="h-9 w-9 rounded-full object-cover shrink-0 grayscale"
            >
            <span
              v-else
              class="h-9 w-9 rounded-full shrink-0 bg-neutral-800 flex items-center justify-center"
            >
              <font-awesome-icon :icon="['fas', 'user']" class="text-neutral-700 text-sm" />
            </span>

            <div class="min-w-0 flex-1">
              <p class="text-gray-300 text-sm truncate">{{ nomComplet(d) }}</p>
              <p v-if="d.retire_at" class="text-xs text-gray-500">
                Retiré le {{ dateCourte(d.retire_at) }}
              </p>
            </div>

            <span
              class="shrink-0 rounded-full border border-white/10 text-gray-400 text-xs px-3 py-1 cursor-help"
              :title="DESCRIPTIONS_ROLE_DETENTEUR[d.role]"
            >
              {{ LIBELLES_ROLE_DETENTEUR[d.role] }}
            </span>
          </li>
        </ul>
      </div>
    </template>
  </section>
</template>
