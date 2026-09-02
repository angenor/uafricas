<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Apporter mon expertise"
        sous-titre="Complétez votre profil et présentez votre expertise. Votre demande sera examinée par un administrateur avant publication."
        image="/images/apporter-expertise.png"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Opafrica', vers: '/actions' },
          { libelle: 'Diapertise', vers: '/experts' },
          { libelle: 'Ma candidature' },
        ]"
      >
        <template v-if="!chargementInit && !succes && !candidatureActive" #centre>
          <p class="text-base font-bold text-af-encre">
            Étape {{ etape + 1 }} sur {{ ETAPES.length }}
          </p>
        </template>
      </AfricansFilAriane>
    </template>

    <div v-if="chargementInit" class="flex flex-col gap-6">
      <div v-for="n in 2" :key="n" class="h-48 animate-pulse rounded-[10px] bg-af-bordure" />
    </div>

    <!-- ═══ Confirmation ═══ -->
    <div v-else-if="succes" class="rounded-[10px] border border-af-vert/30 bg-white p-12 text-center">
      <font-awesome-icon icon="fa-solid fa-circle-check" class="text-5xl text-af-vert" />
      <h1 class="mt-4 text-[24px]/[1.3] font-bold text-af-encre">Demande envoyée</h1>
      <p class="mx-auto mt-2 max-w-lg text-[14px]/[1.5] text-af-corps">
        Elle sera examinée par un administrateur. Vous serez prévenu de la décision.
      </p>
      <div class="mt-6 flex flex-wrap justify-center gap-3">
        <AfricansBouton icone="fa-solid fa-user-tie" vers="/experts">Voir les expertises</AfricansBouton>
        <AfricansBouton variante="secondaire" icone="fa-solid fa-user" vers="/mon-compte/profil">
          Mon profil
        </AfricansBouton>
      </div>
    </div>

    <!-- ═══ Demande déjà déposée ═══ -->
    <div v-else-if="candidatureActive" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
      <font-awesome-icon
        :icon="candidatureActive.statut === 'valide' ? 'fa-solid fa-circle-check' : 'fa-solid fa-clock'"
        class="text-5xl"
        :class="candidatureActive.statut === 'valide' ? 'text-af-vert' : 'text-af-chocolat'"
      />
      <h1 class="mt-4 text-[24px]/[1.3] font-bold text-af-encre">
        {{ candidatureActive.statut === 'valide' ? 'Vous êtes déjà expert' : "Demande en cours d'examen" }}
      </h1>
      <p class="mx-auto mt-2 max-w-lg text-[14px]/[1.5] text-af-corps">
        {{ candidatureActive.statut === 'valide'
          ? 'Votre expertise est publiée dans le répertoire.'
          : 'Un administrateur doit encore l’examiner. Vous serez prévenu de la décision.' }}
      </p>
      <AfricansBouton class="mt-6" icone="fa-solid fa-user-tie" vers="/experts">
        Voir les expertises
      </AfricansBouton>
    </div>

    <!-- ═══ Formulaire ═══ -->
    <form v-else class="flex flex-col gap-6" @submit.prevent="soumettre">
      <div
        v-if="candidatureRefusee"
        class="rounded-[10px] border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.4]"
      >
        <p class="font-bold text-af-live">Votre demande précédente a été refusée.</p>
        <p v-if="candidatureRefusee.commentaireAdmin" class="mt-1 text-af-corps">
          {{ candidatureRefusee.commentaireAdmin }}
        </p>
        <p class="mt-1 text-af-corps">Vous pouvez la corriger et la soumettre à nouveau.</p>
      </div>

      <!-- Fil des étapes -->
      <nav class="flex flex-wrap gap-2" aria-label="Étapes de la candidature">
        <button
          v-for="(e, i) in ETAPES"
          :key="e.titre"
          type="button"
          class="flex min-w-0 flex-1 items-center gap-3 rounded-[10px] border p-3 text-left transition"
          :class="i === etape ? 'border-af-chocolat bg-af-chocolat/[0.07]' : 'border-af-bordure bg-white hover:border-af-chocolat'"
          :aria-current="i === etape ? 'step' : undefined"
          @click="allerA(i)"
        >
          <span
            class="grid size-8 shrink-0 place-items-center rounded-full text-[14px]/[1] font-bold"
            :class="i < etape ? 'bg-af-vert text-white' : i === etape ? 'bg-af-chocolat text-white' : 'bg-af-fond text-af-atone'"
          >
            <font-awesome-icon v-if="i < etape" icon="fa-solid fa-check" />
            <template v-else>{{ i + 1 }}</template>
          </span>
          <span class="min-w-0 truncate text-[14px]/[1.3] font-bold" :class="i === etape ? 'text-af-chocolat' : 'text-af-encre'">
            {{ e.titre }}
          </span>
        </button>
      </nav>

      <p
        v-if="messageEtape || erreurForm"
        role="alert"
        class="flex items-start gap-2 rounded-[10px] border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
      >
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="mt-0.5 shrink-0" />
        {{ messageEtape || erreurForm }}
      </p>

      <section class="flex flex-col gap-5 rounded-[10px] border border-af-bordure bg-white p-6">
        <!-- ─── 1. Mon profil ─── -->
        <template v-if="etape === 0">
          <div class="flex items-center gap-5">
            <AfricansAvatar :nom="form.fonction || 'Photo'" :src="photoPreview || urlMedia(photoUrlActuelle)" :taille="80" />
            <div class="flex flex-col items-start gap-1">
              <AfricansBouton variante="secondaire" icone="fa-solid fa-image" @click="declencherSelectionPhoto">
                Changer la photo
              </AfricansBouton>
              <p class="text-[12px]/[1.4] text-af-atone">JPEG ou PNG, format carré recommandé.</p>
              <input ref="inputPhoto" type="file" accept="image/jpeg,image/png,image/webp" class="hidden" @change="onPhotoChange" />
            </div>
          </div>

          <AfricansChamp v-model="form.fonction" libelle="Fonction *" placeholder="Ex : ingénieure logiciel, médecin, agronome…" />

          <AfricansChamp v-model="form.paysId" libelle="Territoire de résidence *" type="select">
            <option value="">Sélectionner</option>
            <option v-for="p in listePays" :key="p.id" :value="p.id">{{ p.nom }}</option>
          </AfricansChamp>
        </template>

        <!-- ─── 2. Mon expertise ─── -->
        <template v-else-if="etape === 1">
          <AfricansChamp v-model="form.domaine" libelle="Domaine d'expertise *" type="select">
            <option value="">Sélectionner</option>
            <option v-for="d in domainesDisponibles" :key="d" :value="d">{{ d }}</option>
          </AfricansChamp>

          <AfricansChamp
            v-if="form.domaine === DOMAINE_AUTRE"
            v-model="form.domaineAutre"
            libelle="Précisez votre domaine *"
            placeholder="Votre domaine d'expertise"
          />

          <div class="flex flex-col gap-2">
            <span class="text-[14px]/[1.4] text-af-atone italic">Spécialités (facultatives)</span>
            <p class="text-[12px]/[1.4] text-af-atone">
              Sous-domaines ou compétences précises. Quinze au maximum.
            </p>
            <div class="flex gap-2">
              <input
                v-model="specialiteInput"
                type="text"
                placeholder="Ex : développement web, cardiologie…"
                class="h-11 min-w-0 flex-1 rounded-md border border-af-bordure bg-white px-4 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none"
                @keydown.enter.prevent="ajouterSpecialite"
              />
              <AfricansBouton variante="secondaire" icone="fa-solid fa-plus" @click="ajouterSpecialite">Ajouter</AfricansBouton>
            </div>
            <div v-if="form.specialites.length" class="flex flex-wrap gap-2">
              <span
                v-for="(sp, i) in form.specialites"
                :key="sp"
                class="inline-flex items-center gap-2 rounded bg-af-fond px-3 py-1 text-[12px]/[1.4] text-af-encre"
              >
                {{ sp }}
                <button type="button" :aria-label="`Retirer ${sp}`" class="text-af-atone transition hover:text-af-live" @click="retirerSpecialite(i)">
                  <font-awesome-icon icon="fa-solid fa-xmark" />
                </button>
              </span>
            </div>
          </div>

          <AfricansChamp
            v-model="form.biographie"
            libelle="Biographie *"
            type="textarea"
            placeholder="Votre parcours, ce que vous savez faire, ce que vous cherchez à transmettre."
            aide="Entre 10 et 5 000 caractères."
          />
        </template>

        <!-- ─── 3. Mon parcours ─── -->
        <template v-else-if="etape === 2">
          <label class="flex flex-col gap-2">
            <span class="text-[14px]/[1.4] text-af-atone italic">Années d'expérience *</span>
            <!-- Champ natif : `AfricansChamp` n'émet que des chaînes, et l'API
                 attend un entier. `5` partirait en `"5"`. -->
            <input
              v-model.number="form.nbAnnees"
              type="number"
              min="0"
              class="h-11 rounded-md border border-af-bordure bg-white px-4 text-[14px]/[1.4] focus:border-af-chocolat focus:outline-none"
            />
          </label>

          <fieldset class="flex flex-col gap-2">
            <legend class="mb-2 text-[14px]/[1.4] text-af-atone italic">Situation(s) professionnelle(s) *</legend>
            <div class="grid gap-2 sm:grid-cols-2">
              <label
                v-for="s in situationsDisponibles"
                :key="s.id"
                class="flex cursor-pointer items-center gap-3 rounded-lg border px-4 py-2.5 text-[14px]/[1.4] transition"
                :class="form.situations.includes(s.id) ? 'border-af-chocolat bg-af-chocolat/[0.07] font-bold text-af-chocolat' : 'border-af-bordure text-af-corps hover:border-af-chocolat'"
              >
                <input
                  type="checkbox"
                  class="size-4 accent-af-chocolat"
                  :checked="form.situations.includes(s.id)"
                  @change="toggleSituation(s.id)"
                />
                {{ s.label }}
              </label>
            </div>
          </fieldset>

          <fieldset class="flex flex-col gap-2">
            <legend class="mb-2 text-[14px]/[1.4] text-af-atone italic">Objectifs actuels (facultatifs)</legend>
            <div class="flex flex-wrap gap-2">
              <button
                v-for="o in OBJECTIFS_EXPERTISE"
                :key="o.value"
                type="button"
                class="rounded-full px-3 py-1.5 text-[12px]/[1.4] font-bold transition"
                :class="form.objectifs.includes(o.value) ? 'bg-af-chocolat text-white' : 'bg-af-fond text-af-corps hover:bg-af-bordure'"
                :aria-pressed="form.objectifs.includes(o.value)"
                @click="toggleObjectif(o.value)"
              >
                {{ o.label }}
              </button>
            </div>
          </fieldset>

          <div class="flex flex-col gap-2">
            <span class="text-[14px]/[1.4] text-af-atone italic">Réalisations (facultatives)</span>
            <div class="flex gap-2">
              <input
                v-model="realisationInput"
                type="text"
                placeholder="Une réalisation marquante"
                class="h-11 min-w-0 flex-1 rounded-md border border-af-bordure bg-white px-4 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none"
                @keydown.enter.prevent="ajouterRealisation"
              />
              <AfricansBouton variante="secondaire" icone="fa-solid fa-plus" @click="ajouterRealisation">Ajouter</AfricansBouton>
            </div>
            <ul v-if="form.realisations.length" class="flex flex-col gap-1">
              <li
                v-for="(r, i) in form.realisations"
                :key="`${r}-${i}`"
                class="flex items-start gap-2 rounded-lg bg-af-fond px-3 py-2 text-[14px]/[1.4] text-af-corps"
              >
                <span class="min-w-0 flex-1">{{ r }}</span>
                <button type="button" :aria-label="`Retirer la réalisation ${i + 1}`" class="shrink-0 text-af-atone transition hover:text-af-live" @click="retirerRealisation(i)">
                  <font-awesome-icon icon="fa-solid fa-xmark" />
                </button>
              </li>
            </ul>
          </div>
        </template>

        <!-- ─── 4. Mes références ─── -->
        <template v-else>
          <AfricansChamp v-model="form.portfolio" libelle="Portfolio / site web (facultatif)" type="url" placeholder="https://…" />
          <AfricansChamp v-model="form.linkedin" libelle="Profil LinkedIn (facultatif)" type="url" placeholder="https://www.linkedin.com/in/votre-profil" />

          <div class="flex flex-col gap-2">
            <span class="text-[14px]/[1.4] text-af-atone italic">CV (facultatif, PDF)</span>
            <div class="flex flex-wrap items-center gap-3">
              <AfricansBouton variante="secondaire" icone="fa-solid fa-file-pdf" @click="declencherSelectionCV">
                Choisir un fichier
              </AfricansBouton>
              <span class="min-w-0 truncate text-[14px]/[1.4] text-af-atone">
                {{ cvNom || 'Aucun fichier sélectionné' }}
              </span>
              <button v-if="cvNom" type="button" class="text-[14px]/[1.4] font-bold text-af-live transition hover:opacity-70" @click="retirerCV">
                Retirer
              </button>
            </div>
            <input ref="inputCV" type="file" accept="application/pdf" class="hidden" @change="onCVChange" />
            <p class="text-[12px]/[1.4] text-af-atone">PDF uniquement, 10 Mo maximum.</p>
            <p v-if="erreurs.cv" role="alert" class="text-[12px]/[1.4] text-af-live">{{ erreurs.cv }}</p>
          </div>
        </template>
      </section>

      <div class="flex flex-wrap items-center gap-3">
        <AfricansBouton v-if="etape > 0" variante="secondaire" icone="fa-solid fa-arrow-left" @click="precedent">
          Précédent
        </AfricansBouton>
        <AfricansBouton v-if="etape < ETAPES.length - 1" icone="fa-solid fa-arrow-right" @click="suivant">
          Suivant
        </AfricansBouton>
        <AfricansBouton
          v-else
          type="submit"
          :icone="soumission ? 'fa-solid fa-spinner' : 'fa-solid fa-paper-plane'"
          :desactive="soumission"
          :tourne="soumission"
        >
          {{ soumission ? 'Envoi en cours…' : 'Soumettre ma demande' }}
        </AfricansBouton>
        <AfricansBouton class="ml-auto" variante="secondaire" vers="/experts">Annuler</AfricansBouton>
      </div>
    </form>

    <template v-if="!chargementInit && !succes && !candidatureActive" #rail>
      <AfricansPanneau titre="Progression" icone="fa-solid fa-list-check">
        <ol class="flex flex-col gap-1">
          <li v-for="(e, i) in ETAPES" :key="e.titre">
            <button
              type="button"
              class="flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-left text-[14px]/[1.4] transition"
              :class="i === etape ? 'bg-af-chocolat/15 font-bold text-af-chocolat' : 'text-af-corps hover:bg-af-chocolat/[0.07]'"
              @click="allerA(i)"
            >
              <font-awesome-icon :icon="e.icone" class="w-4 shrink-0 text-center" />
              <span class="min-w-0 flex-1 truncate">{{ e.titre }}</span>
              <font-awesome-icon v-if="i < etape" icon="fa-solid fa-check" class="shrink-0 text-af-vert" />
            </button>
          </li>
        </ol>
      </AfricansPanneau>

      <AfricansPanneau titre="Bon à savoir" icone="fa-solid fa-circle-info">
        <ul class="flex flex-col gap-3 text-[14px]/[1.5] text-af-corps">
          <li class="flex gap-3">
            <font-awesome-icon icon="fa-solid fa-eye" class="mt-1 size-3 shrink-0 text-af-chocolat" />
            Votre demande est examinée par un administrateur avant publication.
          </li>
          <li class="flex gap-3">
            <font-awesome-icon icon="fa-solid fa-user" class="mt-1 size-3 shrink-0 text-af-chocolat" />
            La fonction, le territoire et la photo mettent aussi à jour votre profil.
          </li>
          <li class="flex gap-3">
            <font-awesome-icon icon="fa-solid fa-triangle-exclamation" class="mt-1 size-3 shrink-0 text-af-live" />
            La saisie n'est pas conservée si vous quittez la page.
          </li>
        </ul>
      </AfricansPanneau>
    </template>
  </NuxtLayout>
</template>

<script setup lang="ts">
import {
  CATEGORIES_EXPERTISE,
  PROFILS_PROFESSIONNELS,
  OBJECTIFS_EXPERTISE,
  type CandidatureExpertBody,
  type MaCandidatureAPI,
} from '~/composables/useExperts'

/**
 * Apporter mon expertise, porté sur le gabarit et découpé en QUATRE ÉTAPES.
 *
 * La page présentait quatorze champs d'un seul tenant, dont deux listes à
 * étiquettes, deux groupes de cases à cocher et deux téléversements, sur une
 * carte centrée hors du gabarit de la refonte.
 *
 * `valider()` n'est pas dupliqué : il reste l'autorité de la soumission, et
 * chaque étape n'en interroge que SA part. Deux jeux de règles auraient fini
 * par diverger, et l'écart n'aurait été visible qu'au refus du serveur.
 */
definePageMeta({ layout: false })

useHead({ title: 'Apporter mon expertise | AfricanS' })

const router = useRouter()
const { isAuthenticated } = useAuth()
const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const profilComposable = useProfil()
const { creerCandidature, uploaderCV, obtenirMaCandidature } = useExperts()

// ── Données de référence ──
interface PaysOptionAPI { id: string, nom: string }
const listePays = ref<PaysOptionAPI[]>([])
const domainesDisponibles = CATEGORIES_EXPERTISE.filter(d => d !== 'Tout')
const situationsDisponibles = PROFILS_PROFESSIONNELS.filter(s => s.id !== 'tous')

// Valeur sentinelle pour le domaine « Autre » (précision libre)
const DOMAINE_AUTRE = 'Autre'

// ── État de chargement / soumission ──
const chargementInit = ref(true)
const soumission = ref(false)
const erreurForm = ref<string | null>(null)
const succes = ref(false)
const candidatureActive = ref<MaCandidatureAPI | null>(null)
const candidatureRefusee = ref<MaCandidatureAPI | null>(null)

// ── Formulaire ──
const form = reactive({
  fonction: '',
  paysId: '',
  domaine: '',
  domaineAutre: '',
  biographie: '',
  nbAnnees: 0,
  situations: [] as string[],
  objectifs: [] as string[],
  specialites: [] as string[],
  realisations: [] as string[],
  portfolio: '',
  linkedin: '',
})

// ── Saisie des tags (spécialités) et items (réalisations) ──
const specialiteInput = ref('')
const realisationInput = ref('')

function ajouterSpecialite() {
  const valeur = specialiteInput.value.trim()
  if (valeur && !form.specialites.includes(valeur) && form.specialites.length < 15) {
    form.specialites.push(valeur)
  }
  specialiteInput.value = ''
}

function retirerSpecialite(index: number) {
  form.specialites.splice(index, 1)
}

function ajouterRealisation() {
  const valeur = realisationInput.value.trim()
  if (valeur && form.realisations.length < 15) {
    form.realisations.push(valeur)
  }
  realisationInput.value = ''
}

function retirerRealisation(index: number) {
  form.realisations.splice(index, 1)
}

function toggleObjectif(valeur: string) {
  const idx = form.objectifs.indexOf(valeur)
  if (idx === -1) form.objectifs.push(valeur)
  else form.objectifs.splice(idx, 1)
}

// ── CV (PDF, optionnel) ──
const cvFile = ref<File | null>(null)
const cvNom = ref<string | null>(null)
const inputCV = ref<HTMLInputElement | null>(null)

function declencherSelectionCV() {
  inputCV.value?.click()
}

function onCVChange(event: Event) {
  const fichier = (event.target as HTMLInputElement).files?.[0]
  if (!fichier) return
  if (fichier.type !== 'application/pdf') {
    erreurs.cv = 'Le CV doit être un fichier PDF.'
    return
  }
  if (fichier.size > 10 * 1024 * 1024) {
    erreurs.cv = 'Le CV ne doit pas dépasser 10 Mo.'
    return
  }
  delete erreurs.cv
  cvFile.value = fichier
  cvNom.value = fichier.name
}

function retirerCV() {
  cvFile.value = null
  cvNom.value = null
  if (inputCV.value) inputCV.value.value = ''
}

// ── Photo ──
const photoUrlActuelle = ref<string | null>(null)
const photoPreview = ref<string | null>(null)
const photoFile = ref<File | null>(null)
const inputPhoto = ref<HTMLInputElement | null>(null)

function declencherSelectionPhoto() {
  inputPhoto.value?.click()
}

function onPhotoChange(event: Event) {
  const fichier = (event.target as HTMLInputElement).files?.[0]
  if (!fichier) return
  photoFile.value = fichier
  photoPreview.value = URL.createObjectURL(fichier)
}

function toggleSituation(idSituation: string) {
  const idx = form.situations.indexOf(idSituation)
  if (idx === -1) form.situations.push(idSituation)
  else form.situations.splice(idx, 1)
}

// ── Validation client ──
const erreurs = reactive<Record<string, string>>({})

function valider(): boolean {
  Object.keys(erreurs).forEach(k => delete erreurs[k])

  if (!form.fonction.trim()) erreurs.fonction = 'Votre fonction est requise.'
  if (!form.paysId) erreurs.paysId = 'Sélectionnez votre territoire de résidence.'
  if (!form.domaine) erreurs.domaine = 'Choisissez un domaine d\'expertise.'
  else if (form.domaine === DOMAINE_AUTRE && !form.domaineAutre.trim()) {
    erreurs.domaineAutre = 'Précisez votre domaine d\'expertise.'
  }
  else if (form.domaine === DOMAINE_AUTRE && form.domaineAutre.trim().length > 120) {
    erreurs.domaineAutre = 'La précision ne doit pas dépasser 120 caractères.'
  }
  if (form.biographie.trim().length < 10) erreurs.biographie = 'La biographie doit contenir au moins 10 caractères.'
  else if (form.biographie.trim().length > 5000) erreurs.biographie = 'La biographie ne doit pas dépasser 5000 caractères.'
  if (form.nbAnnees < 0 || Number.isNaN(form.nbAnnees)) erreurs.nbAnnees = 'Le nombre d\'années doit être positif.'
  if (form.situations.length === 0) erreurs.situations = 'Sélectionnez au moins une situation professionnelle.'
  if (form.portfolio.trim() && !/^https?:\/\/.+/.test(form.portfolio.trim())) {
    erreurs.portfolio = 'Le lien doit commencer par http:// ou https://'
  }
  if (form.linkedin.trim() && !/^https?:\/\/(www\.)?linkedin\.com\/.+/i.test(form.linkedin.trim())) {
    erreurs.linkedin = 'Indiquez une URL LinkedIn valide (https://www.linkedin.com/…).'
  }

  return Object.keys(erreurs).length === 0
}

// ── Étapes ────────────────────────────────────────────────────────────────
const ETAPES = [
  { titre: 'Mon profil', icone: 'fa-solid fa-user', champs: ['fonction', 'paysId'] },
  { titre: 'Mon expertise', icone: 'fa-solid fa-lightbulb', champs: ['domaine', 'domaineAutre', 'biographie'] },
  { titre: 'Mon parcours', icone: 'fa-solid fa-briefcase', champs: ['nbAnnees', 'situations'] },
  { titre: 'Mes références', icone: 'fa-solid fa-link', champs: ['portfolio', 'linkedin', 'cv'] },
] as const

const etape = ref(0)

/**
 * Les erreurs d'une étape, tirées de la validation d'ENSEMBLE. `valider()`
 * remplit `erreurs` pour tout le formulaire ; on ne lit ici que les clés qui
 * appartiennent à l'étape, sans réécrire une seule règle.
 */
function erreursEtape(i: number): string[] {
  valider()
  const champs = ETAPES[i]!.champs as readonly string[]
  return champs.map(c => erreurs[c]).filter(Boolean) as string[]
}

const messageEtape = ref<string | null>(null)

function suivant() {
  const manque = erreursEtape(etape.value)
  if (manque.length) { messageEtape.value = manque[0]!; return }
  messageEtape.value = null
  etape.value += 1
  if (import.meta.client) window.scrollTo({ top: 0, behavior: 'smooth' })
}

function precedent() {
  messageEtape.value = null
  etape.value -= 1
  if (import.meta.client) window.scrollTo({ top: 0, behavior: 'smooth' })
}

function allerA(i: number) {
  // On ne saute en avant que par des étapes déjà complètes.
  if (i > etape.value) {
    for (let k = etape.value; k < i; k++) {
      const manque = erreursEtape(k)
      if (manque.length) { etape.value = k; messageEtape.value = manque[0]!; return }
    }
  }
  messageEtape.value = null
  etape.value = i
}

// ── Soumission ──
async function soumettre() {
  erreurForm.value = null
  if (!valider()) {
    // Ramener sur l'étape en défaut : un message générique en pied de page
    // n'aide pas quand le champ fautif est trois écrans plus haut.
    const premiere = ETAPES.findIndex((_, i) => erreursEtape(i).length > 0)
    if (premiere >= 0) {
      etape.value = premiere
      messageEtape.value = erreursEtape(premiere)[0] ?? null
      if (import.meta.client) window.scrollTo({ top: 0, behavior: 'smooth' })
    }
    erreurForm.value = 'Veuillez corriger les champs indiqués.'
    return
  }

  messageEtape.value = null
  soumission.value = true
  try {
    // 1. Photo (optionnelle)
    if (photoFile.value) {
      await profilComposable.changerPhoto(photoFile.value)
    }

    // 2. Profil de base (fonction + pays de résidence)
    await profilComposable.modifierProfil({
      fonction: form.fonction.trim(),
      pays_residence_id: form.paysId,
    })

    // 3. CV (optionnel, PDF), uploadé avant la création de la candidature
    let cvUrl: string | undefined
    if (cvFile.value) {
      cvUrl = (await uploaderCV(cvFile.value)) ?? undefined
    }

    // 4. Candidature d'expertise
    const estAutre = form.domaine === DOMAINE_AUTRE
    const body: CandidatureExpertBody = {
      domaine: estAutre ? 'autre' : form.domaine,
      biographie: form.biographie.trim(),
      nb_annees_experience: form.nbAnnees,
      situations_professionnelles: form.situations,
      objectifs: form.objectifs,
      specialites: form.specialites,
      realisations: form.realisations,
    }
    if (estAutre) body.domaine_autre = form.domaineAutre.trim()
    if (form.portfolio.trim()) body.portfolio = form.portfolio.trim()
    if (form.linkedin.trim()) body.linkedin_url = form.linkedin.trim()
    if (cvUrl) body.cv_url = cvUrl

    const resultat = await creerCandidature(body)
    if (!resultat) {
      throw new Error('La demande n\'a pas pu être enregistrée.')
    }

    succes.value = true
    window.scrollTo({ top: 0, behavior: 'smooth' })
  }
  catch (e: any) {
    const message = e?.data?.error || e?.message || 'Une erreur est survenue lors de la soumission.'
    erreurForm.value = message
  }
  finally {
    soumission.value = false
  }
}

// ── Initialisation ──
async function chargerPays() {
  try {
    const reponse = await $fetch<{ success: boolean, data: PaysOptionAPI[] | null }>(
      `${apiBase}/api/pays`,
    )
    if (reponse.success && reponse.data) {
      listePays.value = reponse.data
    }
  }
  catch {
    // non bloquant : la liste reste vide
  }
}

onMounted(async () => {
  if (!isAuthenticated.value) {
    router.replace({ path: '/login', query: { redirect: '/devenir-expert' } })
    return
  }

  await chargerPays()

  // Pré-remplir depuis le profil
  try {
    const profil = await profilComposable.chargerProfil()
    if (profil) {
      form.fonction = profil.fonction ?? ''
      form.paysId = profil.pays_residence_id ?? ''
      photoUrlActuelle.value = profil.photo_url
    }
  }
  catch {
    // non bloquant
  }

  // Vérifier une candidature active
  try {
    const candidature = await obtenirMaCandidature()
    if (candidature) {
      if (candidature.statut === 'refuse') {
        candidatureRefusee.value = candidature
      }
      else {
        candidatureActive.value = candidature
      }
    }
  }
  catch {
    // non bloquant
  }

  chargementInit.value = false
})
</script>
