<script setup lang="ts">
/**
 * Examen d'une proposition de média (US4).
 *
 * **Examen de licéité** : aucune décharge de droits n'est recueillie du
 * contributeur (H-012, décision explicite du commanditaire). L'administrateur
 * est donc SEUL à se prononcer sur les droits d'auteur et l'autorisation de
 * rediffusion : d'où la source et l'auteur déclaré présentés en évidence, avant
 * même les boutons de décision (FR-033).
 */
import {
  LIBELLES_TYPE_OBJET,
  LIBELLES_STATUT,
  ROLES_PARTIE_PRENANTE,
  type PropositionMediaAPI,
} from '~/composables/useMediaProposition'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const { obtenir, valider, rejeter, chargement, erreur } = useAdminMediaPropositions()

const proposition = ref<PropositionMediaAPI | null>(null)
const commentaire = ref('')
const enCours = ref(false)
const messageLocal = ref('')

const LONGUEUR_MIN_MOTIF = 10

useHead(() => ({
  title: `${proposition.value?.donnees.nom || 'Proposition'}, Modération, Administration`,
}))

const charger = async () => {
  proposition.value = await obtenir(id)
}
onMounted(charger)

const estEnAttente = computed(() => proposition.value?.statut === 'en_attente')

const libelleRole = computed(() => {
  const valeur = proposition.value?.donnees.role_partie_prenante
  if (!valeur) return null
  if (valeur === 'autre') {
    return `Autre, ${proposition.value?.donnees.role_partie_prenante_autre ?? '(non précisé)'}`
  }
  return ROLES_PARTIE_PRENANTE.find(r => r.valeur === valeur)?.libelle ?? valeur
})

const media = computed(() => {
  const d = proposition.value?.donnees
  return d?.video_url || d?.audio_url || d?.stream_url || null
})

const validerProposition = async () => {
  if (enCours.value) return
  enCours.value = true
  messageLocal.value = ''
  const res = await valider(id, commentaire.value.trim() || undefined)
  enCours.value = false
  if (res) {
    messageLocal.value = 'Proposition validée : le contenu est publié et son auteur en est désormais propriétaire.'
    await charger()
  }
}

const rejeterProposition = async () => {
  if (enCours.value) return
  const motif = commentaire.value.trim()
  if (motif.length < LONGUEUR_MIN_MOTIF) {
    messageLocal.value = `Le motif du refus doit compter au moins ${LONGUEUR_MIN_MOTIF} caractères, l’auteur doit pouvoir comprendre la décision.`
    return
  }
  enCours.value = true
  messageLocal.value = ''
  const ok = await rejeter(id, motif)
  enCours.value = false
  if (ok) {
    messageLocal.value = 'Proposition refusée. L’auteur en est averti, avec le motif.'
    await charger()
  }
}

const dateFormatee = (iso: string) =>
  new Date(iso).toLocaleDateString('fr-FR', { day: 'numeric', month: 'long', year: 'numeric' })
</script>

<template>
  <div class="p-6 max-w-4xl">
    <NuxtLink to="/admin/medias/propositions" class="btn btn-ghost btn-sm mb-4">
      &#8592; Retour à la file
    </NuxtLink>

    <div v-if="chargement && !proposition" class="flex justify-center py-16">
      <span class="loading loading-spinner loading-lg"></span>
    </div>

    <div v-else-if="!proposition" class="alert alert-error">
      <span>Proposition introuvable.</span>
    </div>

    <template v-else>
      <header class="mb-6">
        <div class="flex items-start justify-between gap-4 flex-wrap">
          <div>
            <h1 class="text-2xl font-bold">{{ proposition.donnees.nom || '(sans nom)' }}</h1>
            <p class="text-sm opacity-70 mt-1">
              {{ LIBELLES_TYPE_OBJET[proposition.type_objet] }} ·
              proposée le {{ dateFormatee(proposition.created_at) }}
            </p>
          </div>
          <span class="badge badge-lg">{{ LIBELLES_STATUT[proposition.statut] }}</span>
        </div>
      </header>

      <!-- ═══ Examen de licéité : en tête, avant toute décision (FR-033) ═══ -->
      <section class="card bg-base-200 border-2 border-warning mb-6">
        <div class="card-body">
          <h2 class="card-title text-base">
            <span class="text-warning">⚠</span> Examen des droits
          </h2>
          <p class="text-sm opacity-80 mb-3">
            Aucune décharge de droits n’a été recueillie auprès du contributeur.
            Il vous revient d’apprécier la licéité de ce contenu et l’autorisation
            de le rediffuser.
          </p>
          <dl class="grid sm:grid-cols-2 gap-4 text-sm">
            <div>
              <dt class="font-semibold opacity-70">Source déclarée</dt>
              <dd class="mt-0.5">{{ proposition.donnees.source_declaree || ', non renseignée, ' }}</dd>
            </div>
            <div>
              <dt class="font-semibold opacity-70">Auteur déclaré</dt>
              <dd class="mt-0.5">{{ proposition.donnees.auteur_declare || ', non renseigné, ' }}</dd>
            </div>
            <div class="sm:col-span-2">
              <dt class="font-semibold opacity-70">Média proposé</dt>
              <dd class="mt-0.5 break-all">
                <a v-if="media" :href="media" target="_blank" rel="noopener noreferrer" class="link link-primary">
                  {{ media }}
                </a>
                <span v-else>aucun média joint</span>
              </dd>
            </div>
          </dl>
        </div>
      </section>

      <!-- Contenu de la proposition -->
      <section class="card bg-base-100 border border-base-300 mb-6">
        <div class="card-body">
          <h2 class="card-title text-base">Contenu proposé</h2>
          <dl class="grid sm:grid-cols-2 gap-4 text-sm">
            <div v-if="proposition.donnees.description" class="sm:col-span-2">
              <dt class="font-semibold opacity-70">Description</dt>
              <dd class="mt-0.5 whitespace-pre-line">{{ proposition.donnees.description }}</dd>
            </div>
            <div v-if="libelleRole">
              <dt class="font-semibold opacity-70">Rôle de partie prenante</dt>
              <dd class="mt-0.5">{{ libelleRole }}</dd>
            </div>
            <div v-if="proposition.donnees.theme_phare_autre">
              <dt class="font-semibold opacity-70">Thème phare</dt>
              <dd class="mt-0.5">{{ proposition.donnees.theme_phare_autre }}</dd>
            </div>
            <div v-if="proposition.donnees.pays">
              <dt class="font-semibold opacity-70">Territoire</dt>
              <dd class="mt-0.5">{{ proposition.donnees.pays }}</dd>
            </div>
            <div v-if="proposition.donnees.image_couverture_url" class="sm:col-span-2">
              <dt class="font-semibold opacity-70">Image de couverture</dt>
              <dd class="mt-2">
                <img :src="proposition.donnees.image_couverture_url" alt="" class="max-h-48 rounded-lg">
              </dd>
            </div>
          </dl>

          <!-- Une station proposée par un membre est TOUJOURS publiée côté
               territoire : la bannière Radio Africans est éditoriale (FR-036). -->
          <p v-if="proposition.type_objet === 'station_radio'" class="text-xs opacity-60 mt-3">
            À la validation, cette station sera publiée sur la page « Radios nationales ».
            Son basculement vers Radio Africans relève d’une décision éditoriale
            ultérieure, depuis la fiche de la station.
          </p>
        </div>
      </section>

      <!-- Auteur et justification -->
      <section class="card bg-base-100 border border-base-300 mb-6">
        <div class="card-body">
          <h2 class="card-title text-base">Contributeur</h2>
          <p class="text-sm">
            {{ proposition.auteur_prenom }} {{ proposition.auteur_nom }}
            <span class="opacity-60">· {{ proposition.auteur_email }}</span>
          </p>
          <div class="mt-3">
            <p class="text-sm font-semibold opacity-70">Motif de la proposition</p>
            <p class="text-sm mt-0.5 whitespace-pre-line">{{ proposition.justification }}</p>
          </div>
        </div>
      </section>

      <!-- Décision -->
      <section v-if="estEnAttente" class="card bg-base-100 border border-base-300">
        <div class="card-body">
          <h2 class="card-title text-base">Décision</h2>
          <label class="label">
            <span class="label-text">
              Commentaire : facultatif pour une validation,
              <strong>obligatoire et motivé</strong> pour un refus
            </span>
          </label>
          <textarea
            v-model="commentaire"
            rows="3"
            class="textarea textarea-bordered w-full"
            placeholder="Expliquez votre décision à l’auteur…"
          ></textarea>

          <div v-if="messageLocal" class="alert alert-info mt-3">
            <span>{{ messageLocal }}</span>
          </div>
          <div v-if="erreur" class="alert alert-error mt-3">
            <span>{{ erreur }}</span>
          </div>

          <div class="card-actions justify-end mt-4">
            <button class="btn btn-error" :disabled="enCours" @click="rejeterProposition">
              Refuser
            </button>
            <button class="btn btn-success" :disabled="enCours" @click="validerProposition">
              Valider et publier
            </button>
          </div>
        </div>
      </section>

      <!-- Décision déjà prise : les états sont terminaux -->
      <section v-else class="card bg-base-200">
        <div class="card-body">
          <h2 class="card-title text-base">Décision prise</h2>
          <p class="text-sm">
            {{ LIBELLES_STATUT[proposition.statut] }}
            <span v-if="proposition.decide_at"> le {{ dateFormatee(proposition.decide_at) }}</span>
            <span v-if="proposition.decideur_nom">
              par {{ proposition.decideur_prenom }} {{ proposition.decideur_nom }}
            </span>
          </p>
          <p v-if="proposition.commentaire_decision" class="text-sm mt-2 whitespace-pre-line">
            <span class="font-semibold">Commentaire :</span> {{ proposition.commentaire_decision }}
          </p>
          <div v-if="messageLocal" class="alert alert-success mt-3">
            <span>{{ messageLocal }}</span>
          </div>
        </div>
      </section>
    </template>
  </div>
</template>
